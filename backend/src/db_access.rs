use redis::Commands;
use redis::{self, Connection, RedisResult};

use tracing::info;

use crate::textchat::{MessageResponse, RoomInfo, User};

const USER_LIST_NAME: &str = "users";
const CHATROOM_LIST_NAME: &str = "rooms";

pub struct RedisConnection {
    connection: Connection,
}

fn get_room_to_user_key(roomId: String) -> String {
    return format!("room:{}:users", roomId);
}

fn get_room_to_message_key(roomId: String) -> String {
    return format!("room:{}:messages", roomId);
}

fn get_user_to_room_key(userId: String) -> String {
    return format!("user:{}:rooms", userId);
}

impl RedisConnection {
    pub fn new() -> RedisResult<RedisConnection> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let connection = client.get_connection()?;

        let redis_connection = RedisConnection { connection };
        Ok(redis_connection)
    }

    // Add users
    pub fn add_user(&mut self, user: User) -> RedisResult<()> {
        // We need to serialize the User struct into json
        let value = serde_json::to_string(&user).unwrap();
        self.connection.rpush(USER_LIST_NAME, value)
    }

    pub fn list_users(&mut self) -> RedisResult<Vec<User>> {
        // If the list with the key of $key doesn't exist,
        // self.connection.lrange(key, 0, -1) returns Ok([])
        let users: Vec<String> = self.connection.lrange(USER_LIST_NAME, 0, -1)?;
        info!("All users are: {:#?}", users);
        let users: Vec<User> = users
            .into_iter()
            .map(|user| {
                let user: User = serde_json::from_str(&user).unwrap();
                return user;
            })
            .collect();

        Ok(users)
    }

    pub fn find_user_by_userId(&mut self, userId: String) -> RedisResult<Option<User>> {
        let users = self.list_users()?;

        for user in users {
            if user.user_id == userId {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    pub fn find_user_by_username(&mut self, username: String) -> RedisResult<Option<User>> {
        let users = self.list_users()?;

        for user in users {
            if user.name == username {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    pub fn create_room(&mut self, room: RoomInfo) -> RedisResult<()> {
        let value = serde_json::to_string(&room).unwrap();
        self.connection.rpush(CHATROOM_LIST_NAME, value)
    }

    pub fn list_rooms(&mut self) -> RedisResult<Vec<RoomInfo>> {
        let rooms: Vec<String> = self.connection.lrange(CHATROOM_LIST_NAME, 0, -1)?;

        let rooms: Vec<RoomInfo> = rooms
            .into_iter()
            .map(|room| {
                let room: RoomInfo = serde_json::from_str(&room).unwrap();
                return room;
            })
            .collect();

        Ok(rooms)
    }

    pub fn find_room_by_roomId(&mut self, roomId: String) -> RedisResult<Option<RoomInfo>> {
        let rooms = self.list_rooms()?;

        for room in rooms {
            if room.room_id == roomId {
                return Ok(Some(room));
            }
        }

        return Ok(None);
    }

    // Before add user to room, check if the room exists or not. If not, then need to call
    // create_room first

    // Used for both createRoom and joinRoom
    pub fn add_user_to_room(&mut self, user: User, room_info: RoomInfo) -> RedisResult<()> {
        let value = serde_json::to_string(&user).unwrap();
        self.connection
            .rpush(get_room_to_user_key(room_info.room_id.clone()), value)?;

        let value = serde_json::to_string(&room_info).unwrap();
        self.connection
            .rpush(get_user_to_room_key(user.user_id), value)
    }

    pub fn list_rooms_by_userId(&mut self, userId: String) -> RedisResult<Vec<RoomInfo>> {
        let user_to_room_key = get_user_to_room_key(userId);

        let rooms: Vec<String> = self.connection.lrange(user_to_room_key, 0, -1)?;

        let rooms: Vec<RoomInfo> = rooms
            .into_iter()
            .map(|room| {
                let room: RoomInfo = serde_json::from_str(&room).unwrap();
                return room;
            })
            .collect();

        Ok(rooms)
    }

    // List users
    pub fn list_users_in_room(&mut self, roomId: String) -> RedisResult<Vec<User>> {
        // If the list with the key of $key doesn't exist,
        // self.connection.lrange(key, 0, -1) returns Ok([])
        let roomId_clone = roomId.clone();

        let users: Vec<String> = self
            .connection
            .lrange(get_room_to_user_key(roomId), 0, -1)?;

        info!("Users in room: {} are : {:#?}", roomId_clone, users);
        let users: Vec<User> = users
            .into_iter()
            .map(|user| {
                let user: User = serde_json::from_str(&user).unwrap();
                return user;
            })
            .collect();

        Ok(users)
    }

    pub fn find_user_in_room(
        &mut self,
        roomId: String,
        userId: String,
    ) -> RedisResult<Option<User>> {
        let users_in_room = self.list_users_in_room(roomId).unwrap();

        let result = users_in_room
            .iter()
            .find(|user| user.user_id == userId)
            .map(|user| user.clone());

        Ok(result)
    }

    // Add message
    pub fn add_message_to_room(
        &mut self,
        user: User,
        message: String,
        roomId: String,
    ) -> RedisResult<()> {
        let message = MessageResponse {
            user_id: user.user_id,
            sender_name: user.name,
            message,
        };
        let value = serde_json::to_string(&message).unwrap();
        self.connection
            .rpush(get_room_to_message_key(roomId), value)
    }

    // List all chat messages
    pub fn list_messages_in_room(&mut self, roomId: String) -> RedisResult<Vec<MessageResponse>> {
        let messages: Vec<String> =
            self.connection
                .lrange(get_room_to_message_key(roomId), 0, -1)?;

        let messages: Vec<MessageResponse> = messages
            .into_iter()
            .map(|message| {
                let message: MessageResponse = serde_json::from_str(&message).unwrap();
                return message;
            })
            .collect();

        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use crate::textchat::User;

    use super::RedisConnection;
    use redis::{Commands, RedisResult};
    use std::collections::HashMap;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_add_and_list_users() {
        let mut redis_connection =
            RedisConnection::new().expect("Fail to create connection to Redis");

        let user_0 = User {
            user_id: "0".to_string(),
            name: "admin".to_string(),
        };

        let result = redis_connection
            .add_user(user_0)
            .expect("Fail to add user to the Redis list");

        let user_1 = User {
            user_id: "1".to_string(),
            name: "admin".to_string(),
        };

        let result = redis_connection
            .add_user(user_1)
            .expect("Fail to add user to the Redis list");

        let users = redis_connection
            .list_users()
            .expect("Fail to list all users");

        println!("Users are {:#?}", users);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn clear_redis_db() {
        let mut redis_connection =
            RedisConnection::new().expect("Fail to create connection to Redis");

        let keys: Vec<String> = redis_connection.connection.keys("*").unwrap();
        println!("Keys are {:#?}", keys);

        for key in keys {
            let _result: u32 = redis_connection.connection.del(key).unwrap();
        }
    }
}
