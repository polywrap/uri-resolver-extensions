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
use crate::ClientResolveResult;

pub fn serialize_client_resolve_result(args: &ClientResolveResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientResolveResult".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_resolve_result(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_resolve_result<W: Write>(args: &ClientResolveResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("cid", "String", "writing property");
    writer.write_string("cid")?;
    writer.write_string(&args.cid)?;
    writer.context().pop();
    writer.context().push("provider", "String", "writing property");
    writer.write_string("provider")?;
    writer.write_string(&args.provider)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_client_resolve_result(args: &[u8]) -> Result<ClientResolveResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientResolveResult".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_resolve_result(&mut reader)
}

pub fn read_client_resolve_result<R: Read>(reader: &mut R) -> Result<ClientResolveResult, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _cid: String = String::new();
    let mut _cid_set = false;
    let mut _provider: String = String::new();
    let mut _provider_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "cid" => {
                reader.context().push(&field, "String", "type found, reading property");
                _cid = reader.read_string()?;
                _cid_set = true;
                reader.context().pop();
            }
            "provider" => {
                reader.context().push(&field, "String", "type found, reading property");
                _provider = reader.read_string()?;
                _provider_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_cid_set {
        return Err(DecodeError::MissingField("cid: String.".to_string()));
    }
    if !_provider_set {
        return Err(DecodeError::MissingField("provider: String.".to_string()));
    }

    Ok(ClientResolveResult {
        cid: _cid,
        provider: _provider,
    })
}
