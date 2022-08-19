use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::http;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SDP {
    // We need to add the rename attributes since the serialized json
    // body sent to us has two fields: type and sdp
    #[serde(rename = "type")]
    pub sdp_type: String,
    #[serde(rename = "sdp")]
    pub sdp_content: String,
}

pub async fn create_offer(
    sdp_content: SDP,
    sdp_1: Arc<RwLock<SDP>>,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let mut sdp_1_guard = sdp_1.write().await;

    println!("Received sdp_offer of {:#?}", sdp_content);

    // sdp_1_guard.sdp_content.is_empty() means the sender is the client 1
    // whose sent sdp_content should be used as the offer
    if sdp_1_guard.sdp_content.is_empty() {
        *sdp_1_guard = sdp_content;
        drop(sdp_1_guard);

        println!("Set offer successfully");
        Ok(Box::new(warp::reply::with_status(
            "Successful",
            http::StatusCode::CREATED,
        )))
    } else {
        // This means the sender is actually client 2 who should use the
        // stored sdp_1_guard.sdp_content as its offer content.

        // That's why we returned the serialized sdp_1_guard content to the client
        let response = serde_json::to_string(&*sdp_1_guard).unwrap();
        println!("There is already an offer");

        Ok(Box::new(warp::reply::json(&response)))
    }
}

pub async fn create_answer(
    sdp_content: SDP,
    sdp_2: Arc<RwLock<SDP>>,
) -> Result<impl warp::Reply, Infallible> {
    println!("Received sdp_answer of {:#?}", sdp_content);

    let mut sdp_2_guard = sdp_2.write().await;
    *sdp_2_guard = sdp_content;
    drop(sdp_2_guard);

    Ok(warp::reply::with_status(
        "Successful",
        http::StatusCode::CREATED,
    ))
}

pub async fn get_answer(
    sdp_1: Arc<RwLock<SDP>>,
    sdp_2: Arc<RwLock<SDP>>,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let mut sdp_1_guard = sdp_1.write().await;
    let mut sdp_2_guard = sdp_2.write().await;

    if sdp_2_guard.sdp_content != "".to_string() {
        let response = serde_json::to_string(&*sdp_2_guard).unwrap();

        println!("Cleanning up existing offer and answer");

        *sdp_1_guard = SDP::default();
        *sdp_2_guard = SDP::default();

        drop(sdp_1_guard);
        drop(sdp_2_guard);

        Ok(Box::new(warp::reply::json(&response)))
    } else {
        // After sending the offer from client 1, it should use long polling
        // to retrieve the answer content. That's why we return Status Code of
        // NO_CONTENT to the client so that it know it needs to call get_answer again.
        Ok(Box::new(http::StatusCode::NO_CONTENT))
    }
}
