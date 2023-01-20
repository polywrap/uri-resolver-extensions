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
use crate::UriResolverMaybeUriOrManifest;

pub fn serialize_uri_resolver_maybe_uri_or_manifest(args: &UriResolverMaybeUriOrManifest) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: UriResolverMaybeUriOrManifest".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_uri_resolver_maybe_uri_or_manifest(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_uri_resolver_maybe_uri_or_manifest<W: Write>(args: &UriResolverMaybeUriOrManifest, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("uri", "Option<String>", "writing property");
    writer.write_string("uri")?;
    writer.write_optional_string(&args.uri)?;
    writer.context().pop();
    writer.context().push("manifest", "Option<Vec<u8>>", "writing property");
    writer.write_string("manifest")?;
    writer.write_optional_bytes(&args.manifest)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_uri_resolver_maybe_uri_or_manifest(args: &[u8]) -> Result<UriResolverMaybeUriOrManifest, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: UriResolverMaybeUriOrManifest".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_uri_resolver_maybe_uri_or_manifest(&mut reader)
}

pub fn read_uri_resolver_maybe_uri_or_manifest<R: Read>(reader: &mut R) -> Result<UriResolverMaybeUriOrManifest, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _uri: Option<String> = None;
    let mut _manifest: Option<Vec<u8>> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "uri" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _uri = reader.read_optional_string()?;
                reader.context().pop();
            }
            "manifest" => {
                reader.context().push(&field, "Option<Vec<u8>>", "type found, reading property");
                _manifest = reader.read_optional_bytes()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(UriResolverMaybeUriOrManifest {
        uri: _uri,
        manifest: _manifest,
    })
}
