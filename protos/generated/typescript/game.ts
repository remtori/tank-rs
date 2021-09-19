/* eslint-disable */
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import * as Long from "long";

export const protobufPackage = "";

export enum Action {
  UNKNOWN = 0,
  SHOOT = 1,
  UNRECOGNIZED = -1,
}

export function actionFromJSON(object: any): Action {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return Action.UNKNOWN;
    case 1:
    case "SHOOT":
      return Action.SHOOT;
    case -1:
    case "UNRECOGNIZED":
    default:
      return Action.UNRECOGNIZED;
  }
}

export function actionToJSON(object: Action): string {
  switch (object) {
    case Action.UNKNOWN:
      return "UNKNOWN";
    case Action.SHOOT:
      return "SHOOT";
    default:
      return "UNKNOWN";
  }
}

export interface ClientMove {
  tick: number;
  id: number;
  sessionId: number;
  x: number;
  y: number;
  z: number;
  pitch: number;
  yaw: number;
  actions: Action[];
}

export interface ServerMove {
  tick: number;
  id: number;
  x: number;
  y: number;
  z: number;
  pitch: number;
  yaw: number;
  actions: Action[];
  rtt: number;
}

const baseClientMove: object = {
  tick: 0,
  id: 0,
  sessionId: 0,
  x: 0,
  y: 0,
  z: 0,
  pitch: 0,
  yaw: 0,
  actions: 0,
};

export const ClientMove = {
  encode(message: ClientMove, writer: Writer = Writer.create()): Writer {
    if (message.tick !== 0) {
      writer.uint32(8).uint64(message.tick);
    }
    if (message.id !== 0) {
      writer.uint32(16).uint32(message.id);
    }
    if (message.sessionId !== 0) {
      writer.uint32(24).uint64(message.sessionId);
    }
    if (message.x !== 0) {
      writer.uint32(33).double(message.x);
    }
    if (message.y !== 0) {
      writer.uint32(41).double(message.y);
    }
    if (message.z !== 0) {
      writer.uint32(49).double(message.z);
    }
    if (message.pitch !== 0) {
      writer.uint32(57).double(message.pitch);
    }
    if (message.yaw !== 0) {
      writer.uint32(65).double(message.yaw);
    }
    writer.uint32(74).fork();
    for (const v of message.actions) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): ClientMove {
    const reader = input instanceof Reader ? input : new Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseClientMove } as ClientMove;
    message.actions = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.tick = longToNumber(reader.uint64() as Long);
          break;
        case 2:
          message.id = reader.uint32();
          break;
        case 3:
          message.sessionId = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.x = reader.double();
          break;
        case 5:
          message.y = reader.double();
          break;
        case 6:
          message.z = reader.double();
          break;
        case 7:
          message.pitch = reader.double();
          break;
        case 8:
          message.yaw = reader.double();
          break;
        case 9:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.actions.push(reader.int32() as any);
            }
          } else {
            message.actions.push(reader.int32() as any);
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ClientMove {
    const message = { ...baseClientMove } as ClientMove;
    message.actions = [];
    if (object.tick !== undefined && object.tick !== null) {
      message.tick = Number(object.tick);
    } else {
      message.tick = 0;
    }
    if (object.id !== undefined && object.id !== null) {
      message.id = Number(object.id);
    } else {
      message.id = 0;
    }
    if (object.sessionId !== undefined && object.sessionId !== null) {
      message.sessionId = Number(object.sessionId);
    } else {
      message.sessionId = 0;
    }
    if (object.x !== undefined && object.x !== null) {
      message.x = Number(object.x);
    } else {
      message.x = 0;
    }
    if (object.y !== undefined && object.y !== null) {
      message.y = Number(object.y);
    } else {
      message.y = 0;
    }
    if (object.z !== undefined && object.z !== null) {
      message.z = Number(object.z);
    } else {
      message.z = 0;
    }
    if (object.pitch !== undefined && object.pitch !== null) {
      message.pitch = Number(object.pitch);
    } else {
      message.pitch = 0;
    }
    if (object.yaw !== undefined && object.yaw !== null) {
      message.yaw = Number(object.yaw);
    } else {
      message.yaw = 0;
    }
    if (object.actions !== undefined && object.actions !== null) {
      for (const e of object.actions) {
        message.actions.push(actionFromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: ClientMove): unknown {
    const obj: any = {};
    message.tick !== undefined && (obj.tick = message.tick);
    message.id !== undefined && (obj.id = message.id);
    message.sessionId !== undefined && (obj.sessionId = message.sessionId);
    message.x !== undefined && (obj.x = message.x);
    message.y !== undefined && (obj.y = message.y);
    message.z !== undefined && (obj.z = message.z);
    message.pitch !== undefined && (obj.pitch = message.pitch);
    message.yaw !== undefined && (obj.yaw = message.yaw);
    if (message.actions) {
      obj.actions = message.actions.map((e) => actionToJSON(e));
    } else {
      obj.actions = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<ClientMove>): ClientMove {
    const message = { ...baseClientMove } as ClientMove;
    message.actions = [];
    if (object.tick !== undefined && object.tick !== null) {
      message.tick = object.tick;
    } else {
      message.tick = 0;
    }
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    } else {
      message.id = 0;
    }
    if (object.sessionId !== undefined && object.sessionId !== null) {
      message.sessionId = object.sessionId;
    } else {
      message.sessionId = 0;
    }
    if (object.x !== undefined && object.x !== null) {
      message.x = object.x;
    } else {
      message.x = 0;
    }
    if (object.y !== undefined && object.y !== null) {
      message.y = object.y;
    } else {
      message.y = 0;
    }
    if (object.z !== undefined && object.z !== null) {
      message.z = object.z;
    } else {
      message.z = 0;
    }
    if (object.pitch !== undefined && object.pitch !== null) {
      message.pitch = object.pitch;
    } else {
      message.pitch = 0;
    }
    if (object.yaw !== undefined && object.yaw !== null) {
      message.yaw = object.yaw;
    } else {
      message.yaw = 0;
    }
    if (object.actions !== undefined && object.actions !== null) {
      for (const e of object.actions) {
        message.actions.push(e);
      }
    }
    return message;
  },
};

const baseServerMove: object = {
  tick: 0,
  id: 0,
  x: 0,
  y: 0,
  z: 0,
  pitch: 0,
  yaw: 0,
  actions: 0,
  rtt: 0,
};

export const ServerMove = {
  encode(message: ServerMove, writer: Writer = Writer.create()): Writer {
    if (message.tick !== 0) {
      writer.uint32(8).uint64(message.tick);
    }
    if (message.id !== 0) {
      writer.uint32(16).uint32(message.id);
    }
    if (message.x !== 0) {
      writer.uint32(25).double(message.x);
    }
    if (message.y !== 0) {
      writer.uint32(33).double(message.y);
    }
    if (message.z !== 0) {
      writer.uint32(41).double(message.z);
    }
    if (message.pitch !== 0) {
      writer.uint32(49).double(message.pitch);
    }
    if (message.yaw !== 0) {
      writer.uint32(57).double(message.yaw);
    }
    writer.uint32(66).fork();
    for (const v of message.actions) {
      writer.int32(v);
    }
    writer.ldelim();
    if (message.rtt !== 0) {
      writer.uint32(72).uint32(message.rtt);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): ServerMove {
    const reader = input instanceof Reader ? input : new Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServerMove } as ServerMove;
    message.actions = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.tick = longToNumber(reader.uint64() as Long);
          break;
        case 2:
          message.id = reader.uint32();
          break;
        case 3:
          message.x = reader.double();
          break;
        case 4:
          message.y = reader.double();
          break;
        case 5:
          message.z = reader.double();
          break;
        case 6:
          message.pitch = reader.double();
          break;
        case 7:
          message.yaw = reader.double();
          break;
        case 8:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.actions.push(reader.int32() as any);
            }
          } else {
            message.actions.push(reader.int32() as any);
          }
          break;
        case 9:
          message.rtt = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ServerMove {
    const message = { ...baseServerMove } as ServerMove;
    message.actions = [];
    if (object.tick !== undefined && object.tick !== null) {
      message.tick = Number(object.tick);
    } else {
      message.tick = 0;
    }
    if (object.id !== undefined && object.id !== null) {
      message.id = Number(object.id);
    } else {
      message.id = 0;
    }
    if (object.x !== undefined && object.x !== null) {
      message.x = Number(object.x);
    } else {
      message.x = 0;
    }
    if (object.y !== undefined && object.y !== null) {
      message.y = Number(object.y);
    } else {
      message.y = 0;
    }
    if (object.z !== undefined && object.z !== null) {
      message.z = Number(object.z);
    } else {
      message.z = 0;
    }
    if (object.pitch !== undefined && object.pitch !== null) {
      message.pitch = Number(object.pitch);
    } else {
      message.pitch = 0;
    }
    if (object.yaw !== undefined && object.yaw !== null) {
      message.yaw = Number(object.yaw);
    } else {
      message.yaw = 0;
    }
    if (object.actions !== undefined && object.actions !== null) {
      for (const e of object.actions) {
        message.actions.push(actionFromJSON(e));
      }
    }
    if (object.rtt !== undefined && object.rtt !== null) {
      message.rtt = Number(object.rtt);
    } else {
      message.rtt = 0;
    }
    return message;
  },

  toJSON(message: ServerMove): unknown {
    const obj: any = {};
    message.tick !== undefined && (obj.tick = message.tick);
    message.id !== undefined && (obj.id = message.id);
    message.x !== undefined && (obj.x = message.x);
    message.y !== undefined && (obj.y = message.y);
    message.z !== undefined && (obj.z = message.z);
    message.pitch !== undefined && (obj.pitch = message.pitch);
    message.yaw !== undefined && (obj.yaw = message.yaw);
    if (message.actions) {
      obj.actions = message.actions.map((e) => actionToJSON(e));
    } else {
      obj.actions = [];
    }
    message.rtt !== undefined && (obj.rtt = message.rtt);
    return obj;
  },

  fromPartial(object: DeepPartial<ServerMove>): ServerMove {
    const message = { ...baseServerMove } as ServerMove;
    message.actions = [];
    if (object.tick !== undefined && object.tick !== null) {
      message.tick = object.tick;
    } else {
      message.tick = 0;
    }
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    } else {
      message.id = 0;
    }
    if (object.x !== undefined && object.x !== null) {
      message.x = object.x;
    } else {
      message.x = 0;
    }
    if (object.y !== undefined && object.y !== null) {
      message.y = object.y;
    } else {
      message.y = 0;
    }
    if (object.z !== undefined && object.z !== null) {
      message.z = object.z;
    } else {
      message.z = 0;
    }
    if (object.pitch !== undefined && object.pitch !== null) {
      message.pitch = object.pitch;
    } else {
      message.pitch = 0;
    }
    if (object.yaw !== undefined && object.yaw !== null) {
      message.yaw = object.yaw;
    } else {
      message.yaw = 0;
    }
    if (object.actions !== undefined && object.actions !== null) {
      for (const e of object.actions) {
        message.actions.push(e);
      }
    }
    if (object.rtt !== undefined && object.rtt !== null) {
      message.rtt = object.rtt;
    } else {
      message.rtt = 0;
    }
    return message;
  },
};

declare var self: any | undefined;
declare var window: any | undefined;
declare var global: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") return globalThis;
  if (typeof self !== "undefined") return self;
  if (typeof window !== "undefined") return window;
  if (typeof global !== "undefined") return global;
  throw "Unable to locate global object";
})();

type Builtin =
  | Date
  | Function
  | Uint8Array
  | string
  | number
  | boolean
  | undefined;
export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
// add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
if (util.Long !== Long) {
  util.Long = Long as any;
  configure();
}
