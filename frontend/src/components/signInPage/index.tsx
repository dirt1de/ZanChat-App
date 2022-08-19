import { Button, Input } from "antd";
import { RpcError } from "grpc-web";
import { useState } from "react";
import { TextChatClient } from "../../proto/TextchatServiceClientPb";
import {
  InitiateChatRequest,
  InitiateChatResponse,
  User,
} from "../../proto/textchat_pb";
import styles from "./index.module.scss";

interface SignInComponentProps {
  setCurrentUser: React.Dispatch<
    React.SetStateAction<User.AsObject | undefined>
  >;
  setIsLoggedIn: React.Dispatch<React.SetStateAction<boolean>>;
  client: TextChatClient;
}

const SignInComponent: React.FC<SignInComponentProps> = ({
  setCurrentUser,
  setIsLoggedIn,
  client,
}) => {
  const [userName, setUserName] = useState("");
  const [userId, setUserId] = useState("");
  const [isLogIn, setIsLogIn] = useState(false);
  const [isSignup, setIsSignup] = useState(false);

  let initiate_chat_response_handling = (
    err: RpcError,
    response: InitiateChatResponse
  ) => {
    if (
      err
      // err.message === "Please make sure both your name and avatar exist"
    ) {
      // do something
      console.error(err);
      alert(err.message);
    } else {
      // This if for the case of no error
      let result = response.toObject();
      console.log("initiate chat result is", result.userid);

      setUserId(result.userid);
      let user = new User();
      user.setUserid(result.userid);

      user.setName(userName);
      setCurrentUser(user.toObject());

      setIsLoggedIn(true);

      console.log(user.toObject());
    }
  };

  let initiate_chat = async (e: any, type: string) => {
    let request = new InitiateChatRequest();
    request.setName(userName);

    // change initiateChat to login and signup

    if (type === "signup") {
      client.signUp(request, {}, (err, response) => {
        initiate_chat_response_handling(err, response);
      });
    } else {
      client.logIn(request, {}, (err, response) => {
        initiate_chat_response_handling(err, response);
      });
    }
    // client.initiateChat(request, {}, (err, response) => {});
  };

  return (
    <div className={styles.formContainer}>
      <div className={styles.loginSignUpWrapper}>
        {!isLogIn ? (
          <Button
            className={styles.buttonWrapper}
            onClick={(e) => {
              setIsLogIn(true);
              setIsSignup(false);
            }}
          >
            Login 🍵
          </Button>
        ) : (
          <div className={styles.inputWrapper}>
            <Input.Group compact>
              <Input
                className={styles.userNameInput}
                value={userName}
                onChange={(e) => {
                  setUserName(e.target.value);
                }}
                onPressEnter={(e) => {
                  initiate_chat(e, "login");
                }}
                placeholder="Login with your username"
              />
              <Button
                className={styles.submitBtn}
                type="primary"
                onClick={(e) => {
                  initiate_chat(e, "login");
                }}
              >
                Submit
              </Button>
            </Input.Group>
          </div>
        )}
        <div style={{ margin: "10px", fontSize: "20px", fontWeight: "500" }}>
          or
        </div>
        {!isSignup ? (
          <Button
            className={styles.buttonWrapper}
            onClick={(e) => {
              setIsSignup(true);
              setIsLogIn(false);
            }}
          >
            Sign up 🆙
          </Button>
        ) : (
          <div className={styles.inputWrapper}>
            <Input.Group compact>
              <Input
                className={styles.userNameInput}
                value={userName}
                onChange={(e) => {
                  setUserName(e.target.value);
                }}
                onPressEnter={(e) => {
                  initiate_chat(e, "signup");
                }}
                placeholder="Create a username"
              />
              <Button
                className={styles.submitBtn}
                type="primary"
                onClick={(e) => {
                  initiate_chat(e, "signup");
                }}
              >
                Submit
              </Button>
            </Input.Group>
          </div>
        )}
      </div>
    </div>
  );
};

export default SignInComponent;
