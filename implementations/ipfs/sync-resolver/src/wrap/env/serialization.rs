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
use crate::Env;

pub fn serialize_env(args: &Env) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) env-type: Env".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_env(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_env<W: Write>(args: &Env, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("timeout", "Option<u32>", "writing property");
    writer.write_string("timeout")?;
    writer.write_optional_u32(&args.timeout)?;
    writer.context().pop();
    writer.context().push("provider", "String", "writing property");
    writer.write_string("provider")?;
    writer.write_string(&args.provider)?;
    writer.context().pop();
    writer.context().push("fallbackProviders", "Option<Vec<String>>", "writing property");
    writer.write_string("fallbackProviders")?;
    writer.write_optional_array(&args.fallback_providers, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_env(args: &[u8]) -> Result<Env, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing env-type: Env".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_env(&mut reader)
}

pub fn read_env<R: Read>(reader: &mut R) -> Result<Env, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _timeout: Option<u32> = None;
    let mut _provider: String = String::new();
    let mut _provider_set = false;
    let mut _fallback_providers: Option<Vec<String>> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "timeout" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _timeout = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "provider" => {
                reader.context().push(&field, "String", "type found, reading property");
                _provider = reader.read_string()?;
                _provider_set = true;
                reader.context().pop();
            }
            "fallbackProviders" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading property");
                _fallback_providers = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_provider_set {
        return Err(DecodeError::MissingField("provider: String.".to_string()));
    }

    Ok(Env {
        timeout: _timeout,
        provider: _provider,
        fallback_providers: _fallback_providers,
    })
}
