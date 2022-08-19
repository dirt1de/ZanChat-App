import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class InitiateChatRequest extends jspb.Message {
  getName(): string;
  setName(value: string): InitiateChatRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InitiateChatRequest.AsObject;
  static toObject(includeInstance: boolean, msg: InitiateChatRequest): InitiateChatRequest.AsObject;
  static serializeBinaryToWriter(message: InitiateChatRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InitiateChatRequest;
  static deserializeBinaryFromReader(message: InitiateChatRequest, reader: jspb.BinaryReader): InitiateChatRequest;
}

export namespace InitiateChatRequest {
  export type AsObject = {
    name: string,
  }
}

export class InitiateChatResponse extends jspb.Message {
  getUserid(): string;
  setUserid(value: string): InitiateChatResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InitiateChatResponse.AsObject;
  static toObject(includeInstance: boolean, msg: InitiateChatResponse): InitiateChatResponse.AsObject;
  static serializeBinaryToWriter(message: InitiateChatResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InitiateChatResponse;
  static deserializeBinaryFromReader(message: InitiateChatResponse, reader: jspb.BinaryReader): InitiateChatResponse;
}

export namespace InitiateChatResponse {
  export type AsObject = {
    userid: string,
  }
}

export class CreateRoomRequest extends jspb.Message {
  getUser(): User | undefined;
  setUser(value?: User): CreateRoomRequest;
  hasUser(): boolean;
  clearUser(): CreateRoomRequest;

  getRoomname(): string;
  setRoomname(value: string): CreateRoomRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateRoomRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateRoomRequest): CreateRoomRequest.AsObject;
  static serializeBinaryToWriter(message: CreateRoomRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateRoomRequest;
  static deserializeBinaryFromReader(message: CreateRoomRequest, reader: jspb.BinaryReader): CreateRoomRequest;
}

export namespace CreateRoomRequest {
  export type AsObject = {
    user?: User.AsObject,
    roomname: string,
  }
}

export class CreateRoomResponse extends jspb.Message {
  getRoomid(): string;
  setRoomid(value: string): CreateRoomResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateRoomResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateRoomResponse): CreateRoomResponse.AsObject;
  static serializeBinaryToWriter(message: CreateRoomResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateRoomResponse;
  static deserializeBinaryFromReader(message: CreateRoomResponse, reader: jspb.BinaryReader): CreateRoomResponse;
}

export namespace CreateRoomResponse {
  export type AsObject = {
    roomid: string,
  }
}

export class JoinRoomRequest extends jspb.Message {
  getUser(): User | undefined;
  setUser(value?: User): JoinRoomRequest;
  hasUser(): boolean;
  clearUser(): JoinRoomRequest;

  getRoomid(): string;
  setRoomid(value: string): JoinRoomRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinRoomRequest.AsObject;
  static toObject(includeInstance: boolean, msg: JoinRoomRequest): JoinRoomRequest.AsObject;
  static serializeBinaryToWriter(message: JoinRoomRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinRoomRequest;
  static deserializeBinaryFromReader(message: JoinRoomRequest, reader: jspb.BinaryReader): JoinRoomRequest;
}

export namespace JoinRoomRequest {
  export type AsObject = {
    user?: User.AsObject,
    roomid: string,
  }
}

export class RoomInfo extends jspb.Message {
  getRoomname(): string;
  setRoomname(value: string): RoomInfo;

  getRoomid(): string;
  setRoomid(value: string): RoomInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RoomInfo.AsObject;
  static toObject(includeInstance: boolean, msg: RoomInfo): RoomInfo.AsObject;
  static serializeBinaryToWriter(message: RoomInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RoomInfo;
  static deserializeBinaryFromReader(message: RoomInfo, reader: jspb.BinaryReader): RoomInfo;
}

export namespace RoomInfo {
  export type AsObject = {
    roomname: string,
    roomid: string,
  }
}

export class ListUserRoomsRequest extends jspb.Message {
  getUserid(): string;
  setUserid(value: string): ListUserRoomsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListUserRoomsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListUserRoomsRequest): ListUserRoomsRequest.AsObject;
  static serializeBinaryToWriter(message: ListUserRoomsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListUserRoomsRequest;
  static deserializeBinaryFromReader(message: ListUserRoomsRequest, reader: jspb.BinaryReader): ListUserRoomsRequest;
}

export namespace ListUserRoomsRequest {
  export type AsObject = {
    userid: string,
  }
}

export class ListUserRoomsResponse extends jspb.Message {
  getRoomsList(): Array<RoomInfo>;
  setRoomsList(value: Array<RoomInfo>): ListUserRoomsResponse;
  clearRoomsList(): ListUserRoomsResponse;
  addRooms(value?: RoomInfo, index?: number): RoomInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListUserRoomsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListUserRoomsResponse): ListUserRoomsResponse.AsObject;
  static serializeBinaryToWriter(message: ListUserRoomsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListUserRoomsResponse;
  static deserializeBinaryFromReader(message: ListUserRoomsResponse, reader: jspb.BinaryReader): ListUserRoomsResponse;
}

export namespace ListUserRoomsResponse {
  export type AsObject = {
    roomsList: Array<RoomInfo.AsObject>,
  }
}

export class MessageRequest extends jspb.Message {
  getUserid(): string;
  setUserid(value: string): MessageRequest;

  getMessage(): string;
  setMessage(value: string): MessageRequest;

  getRoomid(): string;
  setRoomid(value: string): MessageRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MessageRequest.AsObject;
  static toObject(includeInstance: boolean, msg: MessageRequest): MessageRequest.AsObject;
  static serializeBinaryToWriter(message: MessageRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MessageRequest;
  static deserializeBinaryFromReader(message: MessageRequest, reader: jspb.BinaryReader): MessageRequest;
}

export namespace MessageRequest {
  export type AsObject = {
    userid: string,
    message: string,
    roomid: string,
  }
}

export class User extends jspb.Message {
  getUserid(): string;
  setUserid(value: string): User;

  getName(): string;
  setName(value: string): User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): User.AsObject;
  static toObject(includeInstance: boolean, msg: User): User.AsObject;
  static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): User;
  static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
}

export namespace User {
  export type AsObject = {
    userid: string,
    name: string,
  }
}

export class StreamRequest extends jspb.Message {
  getUserid(): string;
  setUserid(value: string): StreamRequest;

  getRoomid(): string;
  setRoomid(value: string): StreamRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StreamRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StreamRequest): StreamRequest.AsObject;
  static serializeBinaryToWriter(message: StreamRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StreamRequest;
  static deserializeBinaryFromReader(message: StreamRequest, reader: jspb.BinaryReader): StreamRequest;
}

export namespace StreamRequest {
  export type AsObject = {
    userid: string,
    roomid: string,
  }
}

export class UsersResponse extends jspb.Message {
  getUser(): User | undefined;
  setUser(value?: User): UsersResponse;
  hasUser(): boolean;
  clearUser(): UsersResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UsersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UsersResponse): UsersResponse.AsObject;
  static serializeBinaryToWriter(message: UsersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UsersResponse;
  static deserializeBinaryFromReader(message: UsersResponse, reader: jspb.BinaryReader): UsersResponse;
}

export namespace UsersResponse {
  export type AsObject = {
    user?: User.AsObject,
  }
}

export class MessageResponse extends jspb.Message {
  getUserid(): string;
  setUserid(value: string): MessageResponse;

  getSenderName(): string;
  setSenderName(value: string): MessageResponse;

  getMessage(): string;
  setMessage(value: string): MessageResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MessageResponse.AsObject;
  static toObject(includeInstance: boolean, msg: MessageResponse): MessageResponse.AsObject;
  static serializeBinaryToWriter(message: MessageResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MessageResponse;
  static deserializeBinaryFromReader(message: MessageResponse, reader: jspb.BinaryReader): MessageResponse;
}

export namespace MessageResponse {
  export type AsObject = {
    userid: string,
    senderName: string,
    message: string,
  }
}

