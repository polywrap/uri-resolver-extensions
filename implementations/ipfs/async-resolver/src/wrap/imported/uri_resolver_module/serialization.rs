use serde::{Serialize, Deserialize};
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

use crate::UriResolverMaybeUriOrManifest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsTryResolveUri {
    pub authority: String,
    pub path: String,
}

pub fn deserialize_try_resolve_uri_args(args: &[u8]) -> Result<ArgsTryResolveUri, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: try_resolve_uri Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _authority: String = String::new();
    let mut _authority_set = false;
    let mut _path: String = String::new();
    let mut _path_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "authority" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _authority = reader.read_string()?;
                _authority_set = true;
                reader.context().pop();
            }
            "path" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _path = reader.read_string()?;
                _path_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_authority_set {
        return Err(DecodeError::MissingField("authority: String.".to_string()));
    }
    if !_path_set {
        return Err(DecodeError::MissingField("path: String.".to_string()));
    }

    Ok(ArgsTryResolveUri {
        authority: _authority,
        path: _path,
    })
}

pub fn serialize_try_resolve_uri_args(args: &ArgsTryResolveUri) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: try_resolve_uri Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_try_resolve_uri_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_try_resolve_uri_args<W: Write>(args: &ArgsTryResolveUri, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("authority", "String", "writing property");
    writer.write_string("authority")?;
    writer.write_string(&args.authority)?;
    writer.context().pop();
    writer.context().push("path", "String", "writing property");
    writer.write_string("path")?;
    writer.write_string(&args.path)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_try_resolve_uri_result(result: &Option<UriResolverMaybeUriOrManifest>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: try_resolve_uri Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_try_resolve_uri_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_try_resolve_uri_result<W: Write>(result: &Option<UriResolverMaybeUriOrManifest>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("tryResolveUri", "Option<UriResolverMaybeUriOrManifest>", "writing result");
    if result.is_some() {
        UriResolverMaybeUriOrManifest::write(result.as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn deserialize_try_resolve_uri_result(result: &[u8]) -> Result<Option<UriResolverMaybeUriOrManifest>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: try_resolve_uri Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("tryResolveUri", "Option<UriResolverMaybeUriOrManifest>", "reading function output");
    let mut object: Option<UriResolverMaybeUriOrManifest> = None;
    if !reader.is_next_nil()? {
        object = Some(UriResolverMaybeUriOrManifest::read(&mut reader)?);
    } else {
        object = None;
    }
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetFile {
    pub path: String,
}

pub fn deserialize_get_file_args(args: &[u8]) -> Result<ArgsGetFile, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: get_file Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _path: String = String::new();
    let mut _path_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "path" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _path = reader.read_string()?;
                _path_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_path_set {
        return Err(DecodeError::MissingField("path: String.".to_string()));
    }

    Ok(ArgsGetFile {
        path: _path,
    })
}

pub fn serialize_get_file_args(args: &ArgsGetFile) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: get_file Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_file_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_file_args<W: Write>(args: &ArgsGetFile, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("path", "String", "writing property");
    writer.write_string("path")?;
    writer.write_string(&args.path)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_file_result(result: &Option<Vec<u8>>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: get_file Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_file_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_file_result<W: Write>(result: &Option<Vec<u8>>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("getFile", "Option<Vec<u8>>", "writing result");
    writer.write_optional_bytes(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_file_result(result: &[u8]) -> Result<Option<Vec<u8>>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: get_file Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("getFile", "Option<Vec<u8>>", "reading function output");
    let res = reader.read_optional_bytes()?;
    reader.context().pop();
    Ok(res)
}
