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
use crate::ClientResolveOptions;

pub fn serialize_client_resolve_options(args: &ClientResolveOptions) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientResolveOptions".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_resolve_options(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_resolve_options<W: Write>(args: &ClientResolveOptions, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("recursive", "Option<bool>", "writing property");
    writer.write_string("recursive")?;
    writer.write_optional_bool(&args.recursive)?;
    writer.context().pop();
    writer.context().push("dhtRecordCount", "Option<i32>", "writing property");
    writer.write_string("dhtRecordCount")?;
    writer.write_optional_i32(&args.dht_record_count)?;
    writer.context().pop();
    writer.context().push("dhtTimeout", "Option<String>", "writing property");
    writer.write_string("dhtTimeout")?;
    writer.write_optional_string(&args.dht_timeout)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_client_resolve_options(args: &[u8]) -> Result<ClientResolveOptions, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientResolveOptions".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_resolve_options(&mut reader)
}

pub fn read_client_resolve_options<R: Read>(reader: &mut R) -> Result<ClientResolveOptions, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _recursive: Option<bool> = None;
    let mut _dht_record_count: Option<i32> = None;
    let mut _dht_timeout: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "recursive" => {
                reader.context().push(&field, "Option<bool>", "type found, reading property");
                _recursive = reader.read_optional_bool()?;
                reader.context().pop();
            }
            "dhtRecordCount" => {
                reader.context().push(&field, "Option<i32>", "type found, reading property");
                _dht_record_count = reader.read_optional_i32()?;
                reader.context().pop();
            }
            "dhtTimeout" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _dht_timeout = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ClientResolveOptions {
        recursive: _recursive,
        dht_record_count: _dht_record_count,
        dht_timeout: _dht_timeout,
    })
}
