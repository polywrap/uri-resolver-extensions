import {
  Read,
  ReadDecoder,
  Write,
  WriteSizer,
  WriteEncoder,
  Box,
  BigInt,
  BigNumber,
  JSON,
  Context
} from "@polywrap/wasm-as";
import * as Types from "..";

export class Args_simpleMethod {
  arg: string;
}

export function deserializesimpleMethodArgs(argsBuf: ArrayBuffer): Args_simpleMethod {
  const context: Context = new Context("Deserializing module-type: simpleMethod Args");
  const reader = new ReadDecoder(argsBuf, context);
  let numFields = reader.readMapLength();

  let _arg: string = "";
  let _argSet: bool = false;

  while (numFields > 0) {
    numFields--;
    const field = reader.readString();

    reader.context().push(field, "unknown", "searching for property type");
    if (field == "arg") {
      reader.context().push(field, "string", "type found, reading property");
      _arg = reader.readString();
      _argSet = true;
      reader.context().pop();
    }
    reader.context().pop();
  }

  if (!_argSet) {
    throw new Error(reader.context().printWithContext("Missing required argument: 'arg: String'"));
  }

  return {
    arg: _arg
  };
}

export function serializesimpleMethodArgs(args: Args_simpleMethod): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: simpleMethod Args");
  const sizer = new WriteSizer(sizerContext);
  writesimpleMethodArgs(sizer, args);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: simpleMethod Args");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  writesimpleMethodArgs(encoder, args);
  return buffer;
}

export function writesimpleMethodArgs(
  writer: Write,
  args: Args_simpleMethod
): void {
  writer.writeMapLength(1);
  writer.context().push("arg", "string", "writing property");
  writer.writeString("arg");
  writer.writeString(args.arg);
  writer.context().pop();
}

export function serializesimpleMethodResult(result: string): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: simpleMethod Result");
  const sizer = new WriteSizer(sizerContext);
  writesimpleMethodResult(sizer, result);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: simpleMethod Result");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  writesimpleMethodResult(encoder, result);
  return buffer;
}

export function writesimpleMethodResult(writer: Write, result: string): void {
  writer.context().push("simpleMethod", "string", "writing property");
  writer.writeString(result);
  writer.context().pop();
}

export function deserializesimpleMethodResult(buffer: ArrayBuffer): string {
  const context: Context = new Context("Deserializing module-type: simpleMethod Result");
  const reader = new ReadDecoder(buffer, context);

  reader.context().push("simpleMethod", "string", "reading function output");
  const res: string = reader.readString();
  reader.context().pop();

  return res;
}
