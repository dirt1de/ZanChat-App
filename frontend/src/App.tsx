import React, { useEffect, useState } from "react";
import "./App.scss";
import ChatRoom from "./components/chatRoom";
import SignInComponent from "./components/signInPage";
import { TextChatClient } from "./proto/TextchatServiceClientPb";
import {
  InitiateChatRequest,
  MessageResponse,
  User,
} from "./proto/textchat_pb";

function App() {
  const endpoint = "https://localhost:8080";
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [currentUser, setCurrentUser] = useState<User.AsObject>();
  const [client, setClient] = useState(
    new TextChatClient("https://localhost:8080")
  );
  // const [messages, setMessages] = useState<MessageResponse.AsObject[]>();
  // const [users, setUsers] = useState<User.AsObject[]>();

  // useEffect(() => {}, []);

  return (
    <div className="App">
      <div className="logo">
        <div className="logoWrapper">
          <div className="zan">Zan</div>
          <div className="chat">Chat</div>
          🍵
        </div>
      </div>
      {isLoggedIn ? (
        <ChatRoom currentUser={currentUser as User.AsObject} client={client} />
      ) : (
        <SignInComponent
          setCurrentUser={setCurrentUser}
          setIsLoggedIn={setIsLoggedIn}
          client={client}
        />
      )}
    </div>
  );
}

// // Old version (Workable)
// function App() {
//   const [show, setShow] = useState(false);
//   const [peerConnection, setPeerConnection] = useState(new RTCPeerConnection());
//   const videoOne = useRef();
//   const videoTwo = useRef();

//   const endpoint = "http://localhost:8080";

//   let localStream;
//   let remoteStream = new MediaStream();

//   const handle_sdp = async () => {
//     const response = await fetch(endpoint + "/create_offer", {
//       method: "POST",
//       headers: {
//         "Content-Type": "application/json",
//       },
//       body: JSON.stringify(peerConnection.localDescription),
//     });

//     if (response.statusText === "Created") {
//       let id = setInterval(async function () {
//         const response = await fetch(endpoint + "/get_answer");
//         console.log(response);

//         if (response.statusText !== "No Content") {
//           const answer = await response.json();

//           console.log("Answer is ", answer);

//           await setAnswer(answer);

//           clearInterval(id);
//         }
//       }, 1000);
//     } else {
//       console.log("resetting the offer content");

//       const data = await response.json();
//       console.log("resetting offer is: ", data);
//       console.log("Trying to create answer");
//       await createAnswer(data);
//     }
//   };

//   const createOffer = async () => {
//     // 1. add event handler to listen for change in icecandidate
//     peerConnection.onicecandidate = async (event) => {
//       if (!event.candidate) {
//         await handle_sdp();
//       }
//     };

//     // 2. set the localDescription
//     let offer = await peerConnection.createOffer();
//     await peerConnection.setLocalDescription(offer);
//     setPeerConnection(peerConnection);
//   };

//   const createAnswer = async (offer) => {
//     await peerConnection.setRemoteDescription(JSON.parse(offer));

//     // 2. listen to change in icecandidates
//     peerConnection.onicecandidate = async (event) => {
//       if (!event.candidate) {
//         const response = await fetch(endpoint + "/create_answer", {
//           method: "POST",
//           headers: {
//             "Content-Type": "application/json",
//           },
//           body: JSON.stringify(peerConnection.localDescription),
//         });

//         console.log(response);
//       }
//     };
//     // 3. set the LocalDescription as the answer
//     let answer = await peerConnection.createAnswer();
//     await peerConnection.setLocalDescription(answer);
//     setPeerConnection(peerConnection);
//     setShow(true);
//   };

//   const setAnswer = async (answer) => {
//     console.log(peerConnection.localDescription);
//     if (!peerConnection.currentRemoteDescription) {
//       await peerConnection.setRemoteDescription(JSON.parse(answer));
//     }
//     setPeerConnection(peerConnection);
//     setShow(true);
//   };

//   useEffect(() => {
//     let init = async () => {
//       localStream = await navigator.mediaDevices.getUserMedia({
//         video: true,
//         audio: false,
//       });
//       videoOne.current.srcObject = localStream;
//       videoTwo.current.srcObject = remoteStream;
//       localStream.getTracks().forEach((track) => {
//         peerConnection.addTrack(track, localStream);
//       });
//       peerConnection.ontrack = (event) => {
//         event.streams[0].getTracks().forEach((track) => {
//           remoteStream.addTrack(track);
//         });
//       };

//       await createOffer();
//     };

//     init();
//   }, []);

//   return (
//     <div className="App">
//       <div className="logo">
//         <div className="logo-wrapper">
//           <div style={{ color: "greenyellow", fontFamily: "sans-serif" }}>
//             Zan
//           </div>
//           <div style={{ color: "white", marginRight: "5px" }}>Chat</div>
//           🍵
//         </div>
//       </div>
//       <div className="video-wrapper">
//         <div id="videos">
//           <video
//             className="video-player"
//             autoPlay
//             playsInline
//             ref={videoOne}
//           ></video>
//           <video
//             className={show ? "video-player" : "video-player-hidden"}
//             autoPlay
//             playsInline
//             ref={videoTwo}
//           ></video>
//         </div>
//       </div>
//       {/* Maybe we can have two modes:
//       1. Friends: When click the link, all sdp transfers will be handled automatically
//       2. Strangers:   */}
//       {/* <div>
//         <button onClick={onClick}>Let's chat</button>
//       </div> */}
//     </div>
//   );
// }

export default App;
