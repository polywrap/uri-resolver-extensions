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
use crate::ClientAddResult;

pub fn serialize_client_add_result(args: &ClientAddResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientAddResult".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_add_result(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_add_result<W: Write>(args: &ClientAddResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("name", "String", "writing property");
    writer.write_string("name")?;
    writer.write_string(&args.name)?;
    writer.context().pop();
    writer.context().push("hash", "String", "writing property");
    writer.write_string("hash")?;
    writer.write_string(&args.hash)?;
    writer.context().pop();
    writer.context().push("size", "String", "writing property");
    writer.write_string("size")?;
    writer.write_string(&args.size)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_client_add_result(args: &[u8]) -> Result<ClientAddResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientAddResult".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_add_result(&mut reader)
}

pub fn read_client_add_result<R: Read>(reader: &mut R) -> Result<ClientAddResult, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _name: String = String::new();
    let mut _name_set = false;
    let mut _hash: String = String::new();
    let mut _hash_set = false;
    let mut _size: String = String::new();
    let mut _size_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "name" => {
                reader.context().push(&field, "String", "type found, reading property");
                _name = reader.read_string()?;
                _name_set = true;
                reader.context().pop();
            }
            "hash" => {
                reader.context().push(&field, "String", "type found, reading property");
                _hash = reader.read_string()?;
                _hash_set = true;
                reader.context().pop();
            }
            "size" => {
                reader.context().push(&field, "String", "type found, reading property");
                _size = reader.read_string()?;
                _size_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_name_set {
        return Err(DecodeError::MissingField("name: String.".to_string()));
    }
    if !_hash_set {
        return Err(DecodeError::MissingField("hash: String.".to_string()));
    }
    if !_size_set {
        return Err(DecodeError::MissingField("size: String.".to_string()));
    }

    Ok(ClientAddResult {
        name: _name,
        hash: _hash,
        size: _size,
    })
}
