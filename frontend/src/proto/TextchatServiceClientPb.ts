/**
 * @fileoverview gRPC-Web generated client stub for textchat
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as proto_textchat_pb from '../proto/textchat_pb';


export class TextChatClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: any; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'text';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname;
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodDescriptorSignUp = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/SignUp',
    grpcWeb.MethodType.UNARY,
    proto_textchat_pb.InitiateChatRequest,
    proto_textchat_pb.InitiateChatResponse,
    (request: proto_textchat_pb.InitiateChatRequest) => {
      return request.serializeBinary();
    },
    proto_textchat_pb.InitiateChatResponse.deserializeBinary
  );

  signUp(
    request: proto_textchat_pb.InitiateChatRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_textchat_pb.InitiateChatResponse>;

  signUp(
    request: proto_textchat_pb.InitiateChatRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.InitiateChatResponse) => void): grpcWeb.ClientReadableStream<proto_textchat_pb.InitiateChatResponse>;

  signUp(
    request: proto_textchat_pb.InitiateChatRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.InitiateChatResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/textchat.TextChat/SignUp',
        request,
        metadata || {},
        this.methodDescriptorSignUp,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/textchat.TextChat/SignUp',
    request,
    metadata || {},
    this.methodDescriptorSignUp);
  }

  methodDescriptorLogIn = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/LogIn',
    grpcWeb.MethodType.UNARY,
    proto_textchat_pb.InitiateChatRequest,
    proto_textchat_pb.InitiateChatResponse,
    (request: proto_textchat_pb.InitiateChatRequest) => {
      return request.serializeBinary();
    },
    proto_textchat_pb.InitiateChatResponse.deserializeBinary
  );

  logIn(
    request: proto_textchat_pb.InitiateChatRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_textchat_pb.InitiateChatResponse>;

  logIn(
    request: proto_textchat_pb.InitiateChatRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.InitiateChatResponse) => void): grpcWeb.ClientReadableStream<proto_textchat_pb.InitiateChatResponse>;

  logIn(
    request: proto_textchat_pb.InitiateChatRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.InitiateChatResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/textchat.TextChat/LogIn',
        request,
        metadata || {},
        this.methodDescriptorLogIn,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/textchat.TextChat/LogIn',
    request,
    metadata || {},
    this.methodDescriptorLogIn);
  }

  methodDescriptorCreateRoom = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/CreateRoom',
    grpcWeb.MethodType.UNARY,
    proto_textchat_pb.CreateRoomRequest,
    proto_textchat_pb.CreateRoomResponse,
    (request: proto_textchat_pb.CreateRoomRequest) => {
      return request.serializeBinary();
    },
    proto_textchat_pb.CreateRoomResponse.deserializeBinary
  );

  createRoom(
    request: proto_textchat_pb.CreateRoomRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_textchat_pb.CreateRoomResponse>;

  createRoom(
    request: proto_textchat_pb.CreateRoomRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.CreateRoomResponse) => void): grpcWeb.ClientReadableStream<proto_textchat_pb.CreateRoomResponse>;

  createRoom(
    request: proto_textchat_pb.CreateRoomRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.CreateRoomResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/textchat.TextChat/CreateRoom',
        request,
        metadata || {},
        this.methodDescriptorCreateRoom,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/textchat.TextChat/CreateRoom',
    request,
    metadata || {},
    this.methodDescriptorCreateRoom);
  }

  methodDescriptorJoinRoom = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/JoinRoom',
    grpcWeb.MethodType.UNARY,
    proto_textchat_pb.JoinRoomRequest,
    google_protobuf_empty_pb.Empty,
    (request: proto_textchat_pb.JoinRoomRequest) => {
      return request.serializeBinary();
    },
    google_protobuf_empty_pb.Empty.deserializeBinary
  );

  joinRoom(
    request: proto_textchat_pb.JoinRoomRequest,
    metadata: grpcWeb.Metadata | null): Promise<google_protobuf_empty_pb.Empty>;

  joinRoom(
    request: proto_textchat_pb.JoinRoomRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  joinRoom(
    request: proto_textchat_pb.JoinRoomRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/textchat.TextChat/JoinRoom',
        request,
        metadata || {},
        this.methodDescriptorJoinRoom,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/textchat.TextChat/JoinRoom',
    request,
    metadata || {},
    this.methodDescriptorJoinRoom);
  }

  methodDescriptorListUserRooms = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/ListUserRooms',
    grpcWeb.MethodType.UNARY,
    proto_textchat_pb.ListUserRoomsRequest,
    proto_textchat_pb.ListUserRoomsResponse,
    (request: proto_textchat_pb.ListUserRoomsRequest) => {
      return request.serializeBinary();
    },
    proto_textchat_pb.ListUserRoomsResponse.deserializeBinary
  );

  listUserRooms(
    request: proto_textchat_pb.ListUserRoomsRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_textchat_pb.ListUserRoomsResponse>;

  listUserRooms(
    request: proto_textchat_pb.ListUserRoomsRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.ListUserRoomsResponse) => void): grpcWeb.ClientReadableStream<proto_textchat_pb.ListUserRoomsResponse>;

  listUserRooms(
    request: proto_textchat_pb.ListUserRoomsRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: proto_textchat_pb.ListUserRoomsResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/textchat.TextChat/ListUserRooms',
        request,
        metadata || {},
        this.methodDescriptorListUserRooms,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/textchat.TextChat/ListUserRooms',
    request,
    metadata || {},
    this.methodDescriptorListUserRooms);
  }

  methodDescriptorSendMessageToRoom = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/SendMessageToRoom',
    grpcWeb.MethodType.UNARY,
    proto_textchat_pb.MessageRequest,
    google_protobuf_empty_pb.Empty,
    (request: proto_textchat_pb.MessageRequest) => {
      return request.serializeBinary();
    },
    google_protobuf_empty_pb.Empty.deserializeBinary
  );

  sendMessageToRoom(
    request: proto_textchat_pb.MessageRequest,
    metadata: grpcWeb.Metadata | null): Promise<google_protobuf_empty_pb.Empty>;

  sendMessageToRoom(
    request: proto_textchat_pb.MessageRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  sendMessageToRoom(
    request: proto_textchat_pb.MessageRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/textchat.TextChat/SendMessageToRoom',
        request,
        metadata || {},
        this.methodDescriptorSendMessageToRoom,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/textchat.TextChat/SendMessageToRoom',
    request,
    metadata || {},
    this.methodDescriptorSendMessageToRoom);
  }

  methodDescriptorGetUsersInRoom = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/GetUsersInRoom',
    grpcWeb.MethodType.SERVER_STREAMING,
    proto_textchat_pb.StreamRequest,
    proto_textchat_pb.UsersResponse,
    (request: proto_textchat_pb.StreamRequest) => {
      return request.serializeBinary();
    },
    proto_textchat_pb.UsersResponse.deserializeBinary
  );

  getUsersInRoom(
    request: proto_textchat_pb.StreamRequest,
    metadata?: grpcWeb.Metadata): grpcWeb.ClientReadableStream<proto_textchat_pb.UsersResponse> {
    return this.client_.serverStreaming(
      this.hostname_ +
        '/textchat.TextChat/GetUsersInRoom',
      request,
      metadata || {},
      this.methodDescriptorGetUsersInRoom);
  }

  methodDescriptorGetMessagesInRoom = new grpcWeb.MethodDescriptor(
    '/textchat.TextChat/GetMessagesInRoom',
    grpcWeb.MethodType.SERVER_STREAMING,
    proto_textchat_pb.StreamRequest,
    proto_textchat_pb.MessageResponse,
    (request: proto_textchat_pb.StreamRequest) => {
      return request.serializeBinary();
    },
    proto_textchat_pb.MessageResponse.deserializeBinary
  );

  getMessagesInRoom(
    request: proto_textchat_pb.StreamRequest,
    metadata?: grpcWeb.Metadata): grpcWeb.ClientReadableStream<proto_textchat_pb.MessageResponse> {
    return this.client_.serverStreaming(
      this.hostname_ +
        '/textchat.TextChat/GetMessagesInRoom',
      request,
      metadata || {},
      this.methodDescriptorGetMessagesInRoom);
  }

}

