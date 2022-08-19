#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct InitiateChatRequest {
    /// Name should be globally unique
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct InitiateChatResponse {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
// message LogInRequest {
//     string userId = 1;
// }

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomRequest {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    #[prost(string, tag = "2")]
    pub room_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomResponse {
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct JoinRoomRequest {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    #[prost(string, tag = "2")]
    pub room_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RoomInfo {
    #[prost(string, tag = "1")]
    pub room_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub room_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ListUserRoomsRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ListUserRoomsResponse {
    #[prost(message, repeated, tag = "1")]
    pub rooms: ::prost::alloc::vec::Vec<RoomInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MessageRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub room_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StreamRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub room_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UsersResponse {
    /// RoomInfo roomInfo = 1;
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MessageResponse {
    /// RoomInfo roomInfo = 1;
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod text_chat_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct TextChatClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TextChatClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TextChatClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TextChatClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TextChatClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn sign_up(
            &mut self,
            request: impl tonic::IntoRequest<super::InitiateChatRequest>,
        ) -> Result<tonic::Response<super::InitiateChatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/SignUp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log_in(
            &mut self,
            request: impl tonic::IntoRequest<super::InitiateChatRequest>,
        ) -> Result<tonic::Response<super::InitiateChatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/LogIn");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_room(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRoomRequest>,
        ) -> Result<tonic::Response<super::CreateRoomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/CreateRoom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn join_room(
            &mut self,
            request: impl tonic::IntoRequest<super::JoinRoomRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/JoinRoom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_user_rooms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserRoomsRequest>,
        ) -> Result<tonic::Response<super::ListUserRoomsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/ListUserRooms");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_message_to_room(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/SendMessageToRoom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_users_in_room(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::UsersResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/GetUsersInRoom");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_messages_in_room(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::MessageResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/textchat.TextChat/GetMessagesInRoom");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod text_chat_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TextChatServer."]
    #[async_trait]
    pub trait TextChat: Send + Sync + 'static {
        async fn sign_up(
            &self,
            request: tonic::Request<super::InitiateChatRequest>,
        ) -> Result<tonic::Response<super::InitiateChatResponse>, tonic::Status>;
        async fn log_in(
            &self,
            request: tonic::Request<super::InitiateChatRequest>,
        ) -> Result<tonic::Response<super::InitiateChatResponse>, tonic::Status>;
        async fn create_room(
            &self,
            request: tonic::Request<super::CreateRoomRequest>,
        ) -> Result<tonic::Response<super::CreateRoomResponse>, tonic::Status>;
        async fn join_room(
            &self,
            request: tonic::Request<super::JoinRoomRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn list_user_rooms(
            &self,
            request: tonic::Request<super::ListUserRoomsRequest>,
        ) -> Result<tonic::Response<super::ListUserRoomsResponse>, tonic::Status>;
        async fn send_message_to_room(
            &self,
            request: tonic::Request<super::MessageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "Server streaming response type for the GetUsersInRoom method."]
        type GetUsersInRoomStream: futures_core::Stream<Item = Result<super::UsersResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn get_users_in_room(
            &self,
            request: tonic::Request<super::StreamRequest>,
        ) -> Result<tonic::Response<Self::GetUsersInRoomStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetMessagesInRoom method."]
        type GetMessagesInRoomStream: futures_core::Stream<Item = Result<super::MessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn get_messages_in_room(
            &self,
            request: tonic::Request<super::StreamRequest>,
        ) -> Result<tonic::Response<Self::GetMessagesInRoomStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TextChatServer<T: TextChat> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TextChat> TextChatServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TextChatServer<T>
    where
        T: TextChat,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/textchat.TextChat/SignUp" => {
                    #[allow(non_camel_case_types)]
                    struct SignUpSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::UnaryService<super::InitiateChatRequest> for SignUpSvc<T> {
                        type Response = super::InitiateChatResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitiateChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_up(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignUpSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/textchat.TextChat/LogIn" => {
                    #[allow(non_camel_case_types)]
                    struct LogInSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::UnaryService<super::InitiateChatRequest> for LogInSvc<T> {
                        type Response = super::InitiateChatResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitiateChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).log_in(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LogInSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/textchat.TextChat/CreateRoom" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRoomSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::UnaryService<super::CreateRoomRequest> for CreateRoomSvc<T> {
                        type Response = super::CreateRoomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRoomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_room(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/textchat.TextChat/JoinRoom" => {
                    #[allow(non_camel_case_types)]
                    struct JoinRoomSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::UnaryService<super::JoinRoomRequest> for JoinRoomSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::JoinRoomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).join_room(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = JoinRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/textchat.TextChat/ListUserRooms" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserRoomsSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::UnaryService<super::ListUserRoomsRequest> for ListUserRoomsSvc<T> {
                        type Response = super::ListUserRoomsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserRoomsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_user_rooms(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListUserRoomsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/textchat.TextChat/SendMessageToRoom" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageToRoomSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::UnaryService<super::MessageRequest> for SendMessageToRoomSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_message_to_room(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMessageToRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/textchat.TextChat/GetUsersInRoom" => {
                    #[allow(non_camel_case_types)]
                    struct GetUsersInRoomSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::ServerStreamingService<super::StreamRequest>
                        for GetUsersInRoomSvc<T>
                    {
                        type Response = super::UsersResponse;
                        type ResponseStream = T::GetUsersInRoomStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_users_in_room(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUsersInRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/textchat.TextChat/GetMessagesInRoom" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessagesInRoomSvc<T: TextChat>(pub Arc<T>);
                    impl<T: TextChat> tonic::server::ServerStreamingService<super::StreamRequest>
                        for GetMessagesInRoomSvc<T>
                    {
                        type Response = super::MessageResponse;
                        type ResponseStream = T::GetMessagesInRoomStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_messages_in_room(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMessagesInRoomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: TextChat> Clone for TextChatServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TextChat> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TextChat> tonic::transport::NamedService for TextChatServer<T> {
        const NAME: &'static str = "textchat.TextChat";
    }
}
