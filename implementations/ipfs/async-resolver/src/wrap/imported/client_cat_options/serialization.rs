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
use crate::ClientCatOptions;

pub fn serialize_client_cat_options(args: &ClientCatOptions) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientCatOptions".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_cat_options(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_cat_options<W: Write>(args: &ClientCatOptions, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("offset", "Option<i32>", "writing property");
    writer.write_string("offset")?;
    writer.write_optional_i32(&args.offset)?;
    writer.context().pop();
    writer.context().push("length", "Option<i32>", "writing property");
    writer.write_string("length")?;
    writer.write_optional_i32(&args.length)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_client_cat_options(args: &[u8]) -> Result<ClientCatOptions, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientCatOptions".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_cat_options(&mut reader)
}

pub fn read_client_cat_options<R: Read>(reader: &mut R) -> Result<ClientCatOptions, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _offset: Option<i32> = None;
    let mut _length: Option<i32> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "offset" => {
                reader.context().push(&field, "Option<i32>", "type found, reading property");
                _offset = reader.read_optional_i32()?;
                reader.context().pop();
            }
            "length" => {
                reader.context().push(&field, "Option<i32>", "type found, reading property");
                _length = reader.read_optional_i32()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ClientCatOptions {
        offset: _offset,
        length: _length,
    })
}
