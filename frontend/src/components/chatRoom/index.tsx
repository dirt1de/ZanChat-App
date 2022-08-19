import { useEffect, useState, useReducer, useRef } from "react";
import { TextChatClient } from "../../proto/TextchatServiceClientPb";
import {
  CreateRoomRequest,
  JoinRoomRequest,
  ListUserRoomsRequest,
  MessageRequest,
  MessageResponse,
  RoomInfo,
  StreamRequest,
  User,
} from "../../proto/textchat_pb";
import styles from "./index.module.scss";
import { Button, Input, Menu } from "antd";
import type { MenuProps } from "antd";

type MenuItem = Required<MenuProps>["items"][number];

function getItem(
  label: React.ReactNode,
  key: React.Key,
  icon?: React.ReactNode,
  children?: MenuItem[],
  type?: "group"
): MenuItem {
  return {
    key,
    icon,
    children,
    label,
    type,
  } as MenuItem;
}

interface ChatRoomProps {
  currentUser: User.AsObject;
  client: TextChatClient;
}

const ChatRoom: React.FC<ChatRoomProps> = ({ currentUser, client }) => {
  const [roomUsersMap, setRoomUsersMap] = useState<
    Map<string, User.AsObject[]>
  >(new Map());
  const [roomMessagesMap, setRoomMessagesMap] = useState<
    Map<string, MessageResponse.AsObject[]>
  >(new Map());
  const [messageContent, setMessageContent] = useState("");
  const [createdRoomName, setCreatedRoomName] = useState("");
  const [joiningRoomId, setJoiningRoomId] = useState("");
  const [roomList, setRoomList] = useState<RoomInfo.AsObject[]>([]);
  const [currentRoom, setCurrentRoom] = useState<RoomInfo.AsObject>(
    new RoomInfo().toObject()
  );
  const [isStreamSet, setIsStreamSet] = useState(false);
  const [any, forceUpdate] = useReducer((num: any) => num + 1, 0);

  const bottomRef = useRef<HTMLDivElement | null>(null);

  function handleChange() {
    forceUpdate();
  }
  const onSelectRoom: MenuProps["onClick"] = (e) => {
    let selectedRoom = roomList.find((room) => room.roomid === e.key);
    selectedRoom && setCurrentRoom(selectedRoom);
    setMessageContent("");
  };

  const sendMessage = () => {
    let request = new MessageRequest();
    request.setUserid(currentUser.userid);
    request.setMessage(messageContent);
    request.setRoomid(currentRoom.roomid);

    client.sendMessageToRoom(request, {}, (err, response) => {
      // consider the case where username is taken
      if (err) {
        console.error(err);
      } else {
        setMessageContent("");
      }
    });
  };

  const listUserRooms = async () => {
    let request = new ListUserRoomsRequest();
    request.setUserid(currentUser.userid);

    client.listUserRooms(request, {}, (err, response) => {
      if (err) {
        console.error(err);
      } else {
        let roomResponse = response.toObject();
        console.log(roomResponse);
        setRoomList(roomResponse.roomsList);
        setCurrentRoom(roomResponse.roomsList[0] || undefined);
      }
    });
  };

  const listMessagesInCurrentRoom = () => {
    let messagesInCurrentRoom =
      currentRoom?.roomid && roomMessagesMap.get(currentRoom.roomid)
        ? (roomMessagesMap.get(
            currentRoom.roomid
          ) as MessageResponse.AsObject[])
        : [];

    return (
      <div>
        {messagesInCurrentRoom.length > 0 ? (
          messagesInCurrentRoom.map((message, index) => {
            return (
              <div
                className={
                  message.senderName == currentUser.name
                    ? styles.selfMessageWrapper
                    : styles.othersMessageWrapper
                }
                key={index}
              >
                <div className={styles.senderName}>{message.senderName}</div>
                <div className={styles.messageBody}>{message.message}</div>
              </div>
            );
          })
        ) : (
          <div className={styles.emptyMessage} key="no-message">
            {" "}
            No Message
          </div>
        )}

        <div ref={bottomRef} />
      </div>
    );
  };

  const createRoom = async () => {
    let request = new CreateRoomRequest();

    let user = new User();
    user.setName(currentUser.name);
    user.setUserid(currentUser.userid);

    request.setUser(user);
    request.setRoomname(createdRoomName);

    client.createRoom(request, {}, async (err, response) => {
      let room = response.toObject();

      console.log(room);

      await setRoomStreams(room.roomid);

      await listUserRooms();
    });

    setCreatedRoomName("");
  };

  const joinRoom = async () => {
    let request = new JoinRoomRequest();

    let user = new User();
    user.setName(currentUser.name);
    user.setUserid(currentUser.userid);

    request.setUser(user);
    request.setRoomid(joiningRoomId);

    client.joinRoom(request, {}, async (err, response) => {
      await setRoomStreams(joiningRoomId);

      await listUserRooms();
    });

    setJoiningRoomId("");
  };

  const items: MenuProps["items"] = [
    getItem(
      "All Chatrooms",
      "roomList",
      null,
      roomList.length
        ? roomList.map((roominfo) => {
            return getItem(
              <div style={{ marginLeft: "24px" }}>{roominfo.roomname}</div>,
              roominfo.roomid
            );
          })
        : [
            getItem(
              <div
                style={{
                  marginLeft: "24px",
                  fontSize: "14px",
                  color: "rgb(71, 184, 5)",
                }}
              >
                No rooms available
              </div>,
              "empty-item"
            ),
          ]
    ),

    getItem("Create Room", "CreateRoom", null, [
      getItem(
        <Input
          className={styles.createRoomWrapper}
          value={createdRoomName}
          onChange={(e) => {
            setCreatedRoomName(e.target.value);
          }}
          onPressEnter={createRoom}
          placeholder="Create room with name"
        />,
        "createRoom"
      ),
    ]),

    getItem("Join Room", "JoinRoom", null, [
      getItem(
        <Input
          className={styles.joinRoomWrapper}
          value={joiningRoomId}
          onChange={(e) => {
            setJoiningRoomId(e.target.value);
          }}
          onPressEnter={joinRoom}
          placeholder="Join a room with its id"
        />,
        "joinRoom"
      ),
    ]),
  ];

  const initRoomStreams = async () => {
    if (roomList && roomList.length) {
      for (let room of roomList) {
        await setRoomStreams(room.roomid);
      }
    }
  };

  const setRoomStreams = async (roomid: string) => {
    let streamRequest = new StreamRequest();
    streamRequest.setUserid(currentUser.userid);
    streamRequest.setRoomid(roomid);

    (() => {
      console.log("Send getUsersInRoom request");
      let userStream = client.getUsersInRoom(streamRequest);
      userStream.on("data", (response) => {
        let userResponse = response.toObject();
        console.log("Receive user ", userResponse);

        if (roomUsersMap.get(roomid)) {
          let oldUsers = roomUsersMap.get(roomid) as User.AsObject[];

          setRoomUsersMap(
            roomUsersMap.set(roomid, [
              ...oldUsers,
              userResponse.user as User.AsObject,
            ])
          );
        } else {
          setRoomUsersMap(
            roomUsersMap.set(roomid, [userResponse.user as User.AsObject])
          );
        }
        handleChange();
      });
    })();

    (() => {
      console.log("Send getMessagesInRoom request");
      let messageStream = client.getMessagesInRoom(streamRequest);
      messageStream.on("data", (response) => {
        let messageResponse = response.toObject();
        console.log("Receive message", messageResponse);

        if (roomMessagesMap.get(roomid)) {
          let oldMessages = roomMessagesMap.get(
            roomid
          ) as MessageResponse.AsObject[];

          setRoomMessagesMap(
            roomMessagesMap.set(roomid, [...oldMessages, messageResponse])
          );
        } else {
          setRoomMessagesMap(roomMessagesMap.set(roomid, [messageResponse]));
        }

        handleChange();
      });
    })();
  };

  useEffect(() => {
    listUserRooms();
  }, []);

  useEffect(() => {
    bottomRef && bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [roomMessagesMap.get(currentRoom?.roomid)]);

  return (
    <div className={styles.displayWrapper}>
      {isStreamSet ? (
        <div style={{ display: "flex", height: "100%" }}>
          <div
            className={styles.roomListWrapper}
            style={{
              display: "flex",
              flexDirection: "column",
            }}
          >
            <div className={styles.roomList}>
              <Menu
                selectedKeys={[currentRoom?.roomid]}
                defaultOpenKeys={["roomList"]}
                mode="inline"
                items={items}
                onClick={onSelectRoom}
              />
            </div>
            {/* Create room */}
          </div>

          <div className={styles.chatRoomWrapper}>
            {currentRoom?.roomid && (
              <div>
                <div className={styles.chatRoomMetaInfo}>
                  {currentRoom.roomname} (
                  {roomUsersMap.get(currentRoom.roomid)?.length})
                </div>
                <div className={styles.chatRoomId}>{currentRoom.roomid}</div>
              </div>
            )}
            <div className={styles.messageList}>
              {listMessagesInCurrentRoom()}
            </div>
            <Input
              size="large"
              className={styles.messageInput}
              value={messageContent}
              onChange={(e) => {
                setMessageContent(e.target.value);
              }}
              onPressEnter={sendMessage}
              placeholder="Enter the message"
              suffix="🍵"
            />
          </div>

          <div className={styles.placeholder}></div>
        </div>
      ) : (
        <div>
          <Button
            className={styles.startChatBtn}
            onClick={async (e) => {
              setIsStreamSet(true);
              await initRoomStreams();
            }}
          >
            Start chat 🍵
          </Button>
        </div>
      )}
    </div>
  );
};

export default ChatRoom;
