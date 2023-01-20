use std::convert::TryFrom;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
};
use crate::ConcurrentTask;

pub fn serialize_concurrent_task(args: &ConcurrentTask) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ConcurrentTask".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_concurrent_task(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_concurrent_task<W: Write>(args: &ConcurrentTask, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("uri", "String", "writing property");
    writer.write_string("uri")?;
    writer.write_string(&args.uri)?;
    writer.context().pop();
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("args", "Vec<u8>", "writing property");
    writer.write_string("args")?;
    writer.write_bytes(&args.args)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_concurrent_task(args: &[u8]) -> Result<ConcurrentTask, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ConcurrentTask".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_concurrent_task(&mut reader)
}

pub fn read_concurrent_task<R: Read>(reader: &mut R) -> Result<ConcurrentTask, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _uri: String = String::new();
    let mut _uri_set = false;
    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _args: Vec<u8> = vec![];
    let mut _args_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "uri" => {
                reader.context().push(&field, "String", "type found, reading property");
                _uri = reader.read_string()?;
                _uri_set = true;
                reader.context().pop();
            }
            "method" => {
                reader.context().push(&field, "String", "type found, reading property");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Vec<u8>", "type found, reading property");
                _args = reader.read_bytes()?;
                _args_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_uri_set {
        return Err(DecodeError::MissingField("uri: String.".to_string()));
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }
    if !_args_set {
        return Err(DecodeError::MissingField("args: Bytes.".to_string()));
    }

    Ok(ConcurrentTask {
        uri: _uri,
        method: _method,
        args: _args,
    })
}
