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
use crate::Retries;

pub fn serialize_retries(args: &Retries) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: Retries".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_retries(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_retries<W: Write>(args: &Retries, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("tryResolveUri", "Option<u32>", "writing property");
    writer.write_string("tryResolveUri")?;
    writer.write_optional_u32(&args.try_resolve_uri)?;
    writer.context().pop();
    writer.context().push("getFile", "Option<u32>", "writing property");
    writer.write_string("getFile")?;
    writer.write_optional_u32(&args.get_file)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_retries(args: &[u8]) -> Result<Retries, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: Retries".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_retries(&mut reader)
}

pub fn read_retries<R: Read>(reader: &mut R) -> Result<Retries, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _try_resolve_uri: Option<u32> = None;
    let mut _get_file: Option<u32> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "tryResolveUri" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _try_resolve_uri = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "getFile" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _get_file = reader.read_optional_u32()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(Retries {
        try_resolve_uri: _try_resolve_uri,
        get_file: _get_file,
    })
}
