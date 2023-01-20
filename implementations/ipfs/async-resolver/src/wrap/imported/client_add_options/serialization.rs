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
use crate::ClientAddOptions;

pub fn serialize_client_add_options(args: &ClientAddOptions) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientAddOptions".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_add_options(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_add_options<W: Write>(args: &ClientAddOptions, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("pin", "Option<bool>", "writing property");
    writer.write_string("pin")?;
    writer.write_optional_bool(&args.pin)?;
    writer.context().pop();
    writer.context().push("onlyHash", "Option<bool>", "writing property");
    writer.write_string("onlyHash")?;
    writer.write_optional_bool(&args.only_hash)?;
    writer.context().pop();
    writer.context().push("wrapWithDirectory", "Option<bool>", "writing property");
    writer.write_string("wrapWithDirectory")?;
    writer.write_optional_bool(&args.wrap_with_directory)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_client_add_options(args: &[u8]) -> Result<ClientAddOptions, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientAddOptions".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_add_options(&mut reader)
}

pub fn read_client_add_options<R: Read>(reader: &mut R) -> Result<ClientAddOptions, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _pin: Option<bool> = None;
    let mut _only_hash: Option<bool> = None;
    let mut _wrap_with_directory: Option<bool> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "pin" => {
                reader.context().push(&field, "Option<bool>", "type found, reading property");
                _pin = reader.read_optional_bool()?;
                reader.context().pop();
            }
            "onlyHash" => {
                reader.context().push(&field, "Option<bool>", "type found, reading property");
                _only_hash = reader.read_optional_bool()?;
                reader.context().pop();
            }
            "wrapWithDirectory" => {
                reader.context().push(&field, "Option<bool>", "type found, reading property");
                _wrap_with_directory = reader.read_optional_bool()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ClientAddOptions {
        pin: _pin,
        only_hash: _only_hash,
        wrap_with_directory: _wrap_with_directory,
    })
}
