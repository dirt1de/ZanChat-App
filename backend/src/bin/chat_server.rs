use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{
    transport::{
        server::{TcpConnectInfo, TlsConnectInfo},
        Identity, Server, ServerTlsConfig,
    },
    Request, Response, Status, Streaming,
};
use tracing::{info, Level};
use tracing_subscriber;
use warp::Filter;
use webrtc_backend::handlers::{create_answer, create_offer, get_answer};

use webrtc_backend::db_access::RedisConnection;
use webrtc_backend::text_chat_service::TextChatService;
use webrtc_backend::textchat::text_chat_server::TextChatServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // todo: Change the log writer below to non-blocking one
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::INFO)
        // build but do not install the subscriber.
        .init();

    let redis_connection = RedisConnection::new()?;
    let redis_connection_clone = Arc::new(RwLock::new(redis_connection));

    let addr = "0.0.0.0:9090".parse()?;
    info!("Server running at {}", addr);

    let cert = tokio::fs::read("localhost.pem").await?;
    let key = tokio::fs::read("localhost-key.pem").await?;

    let identity = Identity::from_pem(cert, key);

    let text_chat_service = TextChatService::new(redis_connection_clone);

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(TextChatServer::new(text_chat_service))
        .serve(addr)
        .await?;

    Ok(())
}

// The REST version of the webrtc backend

//
// fn json_body() -> impl Filter<Extract = (SDP,), Error = warp::Rejection> + Clone {
//     // When accepting a body, we want a JSON body
//     warp::body::json()
// }

// async fn main() {
//     let sdp_1 = Arc::new(RwLock::new(SDP::default()));
//     let sdp_1_clone = sdp_1.clone();
//     let sdp_2 = Arc::new(RwLock::new(SDP::default()));
//     let sdp_2_clone = sdp_2.clone();

//     // We expose three API endpoints:
//     // 1, /create_offer: the client sends the offer sdp to the server for storage
//     let create_offer = warp::post()
//         .and(warp::path("create_offer"))
//         .and(json_body())
//         .and_then(move |sdp_content| create_offer(sdp_content, sdp_1_clone.clone()));

//     // 2. /create_answer: the client sends the answer sdp to the server for storage
//     let create_answer = warp::post()
//         .and(warp::path("create_answer"))
//         .and(json_body())
//         .and_then(move |sdp_content| create_answer(sdp_content, sdp_2_clone.clone()));

//     let sdp_1_clone = sdp_1.clone();
//     let sdp_2_clone = sdp_2.clone();
//     // 3. /get_answer: the client retrieves the answer sdp content
//     let get_answer = warp::path!("get_answer")
//         .and_then(move || get_answer(sdp_1_clone.clone(), sdp_2_clone.clone()));

//     // The following is the cors configurations
//     let cors = warp::cors()
//         .allow_any_origin()
//         .allow_headers(vec![
//             "User-Agent",
//             "Sec-Fetch-Mode",
//             "Referer",
//             "Origin",
//             "Access-Control-Request-Method",
//             "Access-Control-Allow-Origin",
//             "Access-Control-Allow-Credentials",
//             "Access-Control-Request-Headers",
//             "content-type",
//         ])
//         .allow_methods(vec!["POST", "GET"]);
//     let routes = create_offer.or(create_answer).or(get_answer).with(cors);

//     println!("Server running on localhost:8080...");
//     warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
// }
