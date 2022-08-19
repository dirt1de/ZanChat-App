use crate::db_access::RedisConnection;
use crate::text_chat_service::NewMessageInfo::{NewMessageStreamSender, NewMessageStruct};
use crate::text_chat_service::NewUserInfo::{NewUserStreamSender, NewUserStruct};
use crate::textchat::text_chat_server::TextChat;
use crate::textchat::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use uuid::Uuid;

type UsersStreamSender = mpsc::Sender<Result<UsersResponse, Status>>;
type MessagesStreamSender = mpsc::Sender<Result<MessageResponse, Status>>;
type RoomUsersStreamSenderMap = Arc<RwLock<HashMap<String, Vec<(String, UsersStreamSender)>>>>;
type RoomMessagesStreamSenderMap =
    Arc<RwLock<HashMap<String, Vec<(String, MessagesStreamSender)>>>>;

#[derive(Debug)]
pub enum NewUserInfo {
    NewUserStruct(String, User),
    NewUserStreamSender(String, String, UsersStreamSender),
}

#[derive(Debug)]
enum NewMessageInfo {
    NewMessageStruct(String, MessageResponse),
    NewMessageStreamSender(String, String, MessagesStreamSender),
}

pub struct TextChatService {
    // TODO: remove the RwLock over the connection
    redis_connection: Arc<RwLock<RedisConnection>>,
    new_user_info_tx: mpsc::Sender<NewUserInfo>,
    new_message_info_tx: mpsc::Sender<NewMessageInfo>,
}

impl TextChatService {
    pub fn new(redis_connection: Arc<RwLock<RedisConnection>>) -> TextChatService {
        let room_users_stream_map: RoomUsersStreamSenderMap =
            Arc::new(RwLock::new(HashMap::default()));
        let messages_stream_map: RoomMessagesStreamSenderMap =
            Arc::new(RwLock::new(HashMap::default()));

        let (new_user_info_tx, mut new_user_info_rx) = mpsc::channel::<NewUserInfo>(1000);
        let (new_message_info_tx, mut new_message_info_rx) = mpsc::channel::<NewMessageInfo>(1000);

        tokio::spawn(async move {
            info!("PubSub on new user started");

            let room_users_stream_map_clone = room_users_stream_map.clone();

            while let Some(new_user_info) = new_user_info_rx.recv().await {
                match new_user_info {
                    NewUserStruct(room_id, new_user) => {
                        // Monitor new user
                        // broadcast to the RoomUsersStreamSenderMap
                        info!("Receiving new user in room: {}", room_id);

                        if let Err(user_id) = Self::broadcast_new_user_to_room(
                            room_id,
                            new_user,
                            room_users_stream_map_clone.clone(),
                        )
                        .await
                        {
                            info!("User with id {} is not online", user_id);
                            // This is for the case where user with user_id has dropped the rx side
                            let mut room_users_stream_map_guard =
                                room_users_stream_map_clone.write().await;
                            room_users_stream_map_guard.remove(&user_id);
                        }
                    }
                    NewUserStreamSender(room_id, user_id, new_user_sender) => {
                        info!("Receiving new user sender with user_id: {}", user_id);
                        let mut room_users_stream_map_guard =
                            room_users_stream_map_clone.write().await;

                        // Check if the room exists or not
                        // If exists, then just edit the values
                        // Else, do the insert
                        if let Some(sender_wrapper) = room_users_stream_map_guard.get_mut(&room_id)
                        {
                            sender_wrapper.push((user_id, new_user_sender));

                            info!("Senders in room {} are {:#?}", room_id, sender_wrapper);
                        } else {
                            let room_id_clone = room_id.clone();
                            info!("Creating room_users_stream_map for room: {}", room_id_clone);
                            room_users_stream_map_guard
                                .insert(room_id, vec![(user_id, new_user_sender)]);

                            info!(
                                "Creating room_users_stream_map for room: {} finished",
                                room_id_clone
                            );
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            info!("PubSub on new message started");

            let messages_stream_map_clone = messages_stream_map.clone();
            while let Some(new_message_info) = new_message_info_rx.recv().await {
                info!("Receive new message in pubsub on new message");
                match new_message_info {
                    NewMessageStruct(room_id, new_message) => {
                        // Monitor new message
                        // broadcast to the RoomMessagesStreamSenderMap
                        info!("Receiving new message in room: {}", room_id);

                        if let Err(user_id) = Self::broadcast_new_message_to_room(
                            room_id,
                            new_message,
                            messages_stream_map_clone.clone(),
                        )
                        .await
                        {
                            // This is for the case where user with user_id has dropped the rx side
                            let mut messages_stream_map_guard =
                                messages_stream_map_clone.write().await;
                            messages_stream_map_guard.remove(&user_id);
                        }
                    }
                    NewMessageStreamSender(room_id, user_id, new_message_sender) => {
                        let mut messages_stream_map_guard = messages_stream_map_clone.write().await;

                        if let Some(sender_wrapper) = messages_stream_map_guard.get_mut(&room_id) {
                            sender_wrapper.push((user_id, new_message_sender))
                        } else {
                            messages_stream_map_guard
                                .insert(room_id, vec![(user_id, new_message_sender)]);
                        }
                    }
                }
            }
        });

        TextChatService {
            redis_connection,
            new_user_info_tx,
            new_message_info_tx,
        }
    }

    async fn broadcast_new_user_to_room(
        room_id: String,
        new_user: User,
        room_users_stream_map: RoomUsersStreamSenderMap,
    ) -> Result<(), String> {
        let room_user_stream_map_guard = room_users_stream_map.read().await;
        let senders = room_user_stream_map_guard.get(&room_id).unwrap();
        for (user_id, sender) in senders {
            let new_user_clone = new_user.clone();
            info!(
                "Broadcasting to user with id: {} in room with id: {}",
                user_id, room_id
            );
            if let Err(_err) = sender
                .send(Ok(UsersResponse {
                    user: Some(new_user_clone),
                }))
                .await
            {
                error!("Fail to broadcast new user to user with id: {}", user_id);
                return Err(user_id.to_string());
            }
        }

        Ok(())
    }

    async fn broadcast_new_message_to_room(
        room_id: String,
        new_message: MessageResponse,
        room_messages_stream_map: RoomMessagesStreamSenderMap,
    ) -> Result<(), String> {
        let room_messages_stream_map_guard = room_messages_stream_map.read().await;
        let senders = room_messages_stream_map_guard.get(&room_id).unwrap();

        info!(
            "All message sender in room: {} are: {:#?}",
            room_id, senders
        );
        for (user_id, sender) in senders {
            let new_message_clone = new_message.clone();
            if let Err(err) = sender.send(Ok(new_message_clone)).await {
                error!(
                    "Fail to broadcast new message to user with id: {} because: {}",
                    user_id, err
                );
                return Err(user_id.to_string());
            }
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl TextChat for TextChatService {
    async fn sign_up(
        &self,
        request: Request<InitiateChatRequest>,
    ) -> Result<Response<InitiateChatResponse>, Status> {
        info!("Received InitiateChat request");

        let request = request.into_inner();

        // Before accessing the Redis, check if request is valid
        if request.name.is_empty() {
            return Err(Status::invalid_argument("Please enter your name"));
        }

        // Next check Redis for the request.user
        let mut redis_connection_guard = self.redis_connection.write().await;

        // - If the username doesn't exist, create a new user
        // - If the username exists, return an error
        if let Some(user) = redis_connection_guard
            .find_user_by_username(request.name.clone())
            .unwrap()
        {
            return Err(Status::invalid_argument("This username has been taken"));
        } else {
            let id = Uuid::new_v4().to_string();

            redis_connection_guard
                .add_user(User {
                    user_id: id.clone(),
                    name: request.name.clone(),
                })
                .expect(&format!(
                    "Fail to add user with name of {} into Redis",
                    request.name
                ));

            info!("Finishes sign up");

            let response = InitiateChatResponse { user_id: id };

            Ok(Response::new(response))
        }
    }

    async fn log_in(
        &self,
        request: Request<InitiateChatRequest>,
    ) -> Result<Response<InitiateChatResponse>, Status> {
        info!("Received InitiateChat request");

        let request = request.into_inner();

        // Before accessing the Redis, check if request is valid
        if request.name.is_empty() {
            return Err(Status::invalid_argument("Please enter your name"));
        }

        // Next check Redis for the request.user
        let mut redis_connection_guard = self.redis_connection.write().await;

        // - If the username doesn't exist, create a new user
        // - If the username exists, return an error
        if let Some(user) = redis_connection_guard
            .find_user_by_username(request.name.clone())
            .unwrap()
        {
            // For test purpose, we don't need to deal with duplicate user_name
            let response = InitiateChatResponse {
                user_id: user.user_id,
            };

            info!("Finishes log in");
            Ok(Response::new(response))
        } else {
            return Err(Status::invalid_argument("This username doesn't exit"));
        }
    }

    // After calling create_room, need to call
    // getUsersInRoom and getMessagesInRoom
    async fn create_room(
        &self,
        request: Request<CreateRoomRequest>,
    ) -> Result<Response<CreateRoomResponse>, Status> {
        let request = request.into_inner();

        if request.room_name.is_empty() {
            return Err(Status::invalid_argument("Please enter your the room name"));
        }

        let roomId = Uuid::new_v4().to_string();

        let room_info = RoomInfo {
            room_name: request.room_name,
            room_id: roomId.clone(),
        };

        info!("Creating room: {}", room_info.room_name);
        let mut redis_connection_guard = self.redis_connection.write().await;

        redis_connection_guard
            .create_room(room_info.clone())
            .expect(&format!(
                "Fail to create room with name: {} and id: {}",
                room_info.room_name, room_info.room_id
            ));

        info!("Creating room: {} finished", room_info.room_name);

        let user = request.user.unwrap();

        let user_id = user.user_id.clone();

        info!(
            "Adding user: {} to room: {}",
            user.name, room_info.room_name
        );

        redis_connection_guard
            .add_user_to_room(user.clone(), room_info.clone())
            .expect(&format!(
                "Fail to add user: {} room with name: {}",
                user_id,
                roomId.clone()
            ));
        info!(
            "Adding user: {} to room: {} finished",
            user.name, room_info.room_name
        );
        let response = CreateRoomResponse { room_id: roomId };
        Ok(Response::new(response))
    }

    // We need to broadcast the new user to the room
    async fn join_room(&self, request: Request<JoinRoomRequest>) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        info!("Receive join room request");
        let user = request.user.unwrap();
        let room_id = request.room_id;

        info!("Validating the join room request");
        let mut redis_connection_guard = self.redis_connection.write().await;

        let room_info = redis_connection_guard
            .find_room_by_roomId(room_id.clone())
            .unwrap();

        if let None = room_info {
            error!("The room to be joined doesn't exist");
            return Err(Status::invalid_argument(
                "Please make sure enter the correct room_id",
            ));
        }

        let room_info = room_info.unwrap();

        if user.user_id.is_empty() || user.name.is_empty() || room_id.is_empty() {
            error!("Argument for join_room request is invalid");
            return Err(Status::invalid_argument(
                "Please make sure both the user_id and room_id is non-empty",
            ));
        }

        if let Some(_user) = redis_connection_guard
            .find_user_in_room(room_id.clone(), user.user_id.clone())
            .unwrap()
        {
            return Err(Status::invalid_argument("This user is already in the room"));
        }

        let user_id = user.user_id.clone();
        let user_name = user.name.clone();

        info!("Adding user to room after validation");
        redis_connection_guard
            .add_user_to_room(user, room_info.clone())
            .expect(&format!(
                "Fail to add user: {} room with name: {}",
                user_id, room_id
            ));

        info!("Broadcast new user");
        // Broadcast only when new user join a chatroom
        self.new_user_info_tx
            .send(NewUserInfo::NewUserStruct(
                room_id,
                User {
                    user_id: user_id.clone(),
                    name: user_name.clone(),
                },
            ))
            .await
            .expect(&format!("Fail to send new user with name of {}", user_name));

        Ok(Response::new(()))
    }

    async fn list_user_rooms(
        &self,
        request: Request<ListUserRoomsRequest>,
    ) -> Result<Response<ListUserRoomsResponse>, Status> {
        let request = request.into_inner();

        info!("Listing user rooms for user {}", request.user_id);
        let mut redis_connection_guard = self.redis_connection.write().await;
        let rooms = redis_connection_guard
            .list_rooms_by_userId(request.user_id.clone())
            .expect(&format!(
                "Fail to list the chatrooms for user with id: {}",
                request.user_id
            ));
        info!("Listing user rooms for user {} finished", request.user_id);

        drop(redis_connection_guard);

        let response = ListUserRoomsResponse { rooms };
        return Ok(Response::new(response));
    }

    async fn send_message_to_room(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<()>, Status> {
        // validate request
        let request = request.into_inner();

        if request.user_id.is_empty() || request.message.is_empty() || request.room_id.is_empty() {
            return Err(Status::invalid_argument(
                "id, message body, and room_id should not be empty",
            ));
        }

        let mut redis_connection_guard = self.redis_connection.write().await;

        let sender = redis_connection_guard
            .find_user_by_userId(request.user_id.clone())
            .expect(&format!(
                "Fail to retrieve user information with id of {} from Redis",
                request.user_id
            ));

        if let Some(sender) = sender {
            // deal with the add_message failure
            redis_connection_guard
                .add_message_to_room(
                    sender.clone(),
                    request.message.clone(),
                    request.room_id.clone(),
                )
                .expect("Fail to add message into Redis");

            let message = MessageResponse {
                user_id: sender.user_id,
                sender_name: sender.name.clone(),
                message: request.message,
            };

            self.new_message_info_tx
                .send(NewMessageInfo::NewMessageStruct(request.room_id, message))
                .await
                .expect(&format!(
                    "Fail to send new message sent by user with name name of {}",
                    sender.name
                ));
        } else {
            return Err(Status::invalid_argument(&format!(
                "User with id {} not found",
                request.user_id
            )));
        }

        Ok(Response::new(()))
    }

    type GetUsersInRoomStream = ReceiverStream<Result<UsersResponse, Status>>;
    async fn get_users_in_room(
        &self,
        request: Request<StreamRequest>,
    ) -> Result<Response<Self::GetUsersInRoomStream>, Status> {
        // 1. Create a channel
        let (tx, rx) = mpsc::channel::<Result<UsersResponse, Status>>(100);
        let tx_clone = tx.clone();

        let request = request.into_inner();
        let redis_connection = self.redis_connection.clone();
        let new_user_info_tx = self.new_user_info_tx.clone();

        // 2. Handle the stream
        tokio::spawn(async move {
            let mut redis_connection_guard = redis_connection.write().await;

            info!("Searching user in get_users_in_room");
            let result = redis_connection_guard
                .find_user_by_userId(request.user_id.clone())
                .expect(&format!(
                    "Fail to retrieve user information with id of {} from Redis",
                    request.user_id
                ));

            match result {
                Some(user) => {
                    info!("Searching all uers in get_users_in_room");
                    let users = redis_connection_guard
                        .list_users_in_room(request.room_id.clone())
                        .expect("Fail to retrieve all users from Redis");

                    drop(redis_connection_guard);
                    // let roomInfo = redis_connection_guard.find_room_by_roomId(roomId)
                    info!("Sending response to all uers in get_users_in_room");

                    for user in users {
                        tx.send(Ok(UsersResponse { user: Some(user) }))
                            .await
                            .unwrap();
                    }
                    info!("Sending response to all uers in get_users_in_room finished");

                    // Edit he broadcast logic by including room_id
                    info!("Broadcasting new user sender to all uers in get_users_in_room");

                    new_user_info_tx
                        .send(NewUserInfo::NewUserStreamSender(
                            request.room_id,
                            user.user_id,
                            tx_clone,
                        ))
                        .await;
                    info!("Broadcasting new user sender to all uers in get_users_in_room finished");
                }
                None => {
                    tx.send(Err(Status::invalid_argument(&format!(
                        "User with id {} not found",
                        request.user_id
                    ))))
                    .await
                    .unwrap();
                }
            }
        });
        // 3. Return the stream receiver to the client
        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    type GetMessagesInRoomStream = ReceiverStream<Result<MessageResponse, Status>>;

    async fn get_messages_in_room(
        &self,
        request: Request<StreamRequest>,
    ) -> Result<Response<Self::GetMessagesInRoomStream>, Status> {
        // 1. Create a channel
        let (tx, rx) = mpsc::channel::<Result<MessageResponse, Status>>(100);
        let tx_clone = tx.clone();

        let request = request.into_inner();
        let redis_connection = self.redis_connection.clone();
        let new_message_info_tx = self.new_message_info_tx.clone();

        // 2. Handle the stream
        tokio::spawn(async move {
            let mut redis_connection_guard = redis_connection.write().await;

            let result = redis_connection_guard
                .find_user_by_userId(request.user_id.clone())
                .expect(&format!(
                    "Fail to retrieve user information with id of {} from Redis",
                    request.user_id
                ));
            match result {
                Some(user) => {
                    let messages = redis_connection_guard
                        .list_messages_in_room(request.room_id.clone())
                        .expect("Fail to retrieve all messages from Redis");

                    for message in messages {
                        tx.send(Ok(message)).await.unwrap();
                    }

                    // For each room, save the sender for each user
                    new_message_info_tx
                        .send(NewMessageInfo::NewMessageStreamSender(
                            request.room_id,
                            user.user_id,
                            tx_clone,
                        ))
                        .await;
                }
                None => {
                    tx.send(Err(Status::invalid_argument(&format!(
                        "User with id {} not found",
                        request.user_id
                    ))))
                    .await
                    .unwrap();
                }
            }
        });

        // 3. Return the stream receiver to the client
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
