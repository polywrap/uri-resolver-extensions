use serde::{Serialize, Deserialize};
pub mod serialization;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    DecodeError,
    EncodeError,
    Read,
    Write,
    JSON,
};
pub use serialization::{
    deserialize_uri_resolver_maybe_uri_or_manifest,
    read_uri_resolver_maybe_uri_or_manifest,
    serialize_uri_resolver_maybe_uri_or_manifest,
    write_uri_resolver_maybe_uri_or_manifest
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UriResolverMaybeUriOrManifest {
    pub uri: Option<String>,
    pub manifest: Option<Vec<u8>>,
}

impl UriResolverMaybeUriOrManifest {
    pub const URI: &'static str = "wrap://ens/wrappers.polywrap.eth:uri-resolver-ext@1.1.0";

    pub fn new() -> UriResolverMaybeUriOrManifest {
        UriResolverMaybeUriOrManifest {
            uri: None,
            manifest: None,
        }
    }

    pub fn to_buffer(args: &UriResolverMaybeUriOrManifest) -> Result<Vec<u8>, EncodeError> {
        serialize_uri_resolver_maybe_uri_or_manifest(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<UriResolverMaybeUriOrManifest, DecodeError> {
        deserialize_uri_resolver_maybe_uri_or_manifest(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &UriResolverMaybeUriOrManifest, writer: &mut W) -> Result<(), EncodeError> {
        write_uri_resolver_maybe_uri_or_manifest(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<UriResolverMaybeUriOrManifest, DecodeError> {
        read_uri_resolver_maybe_uri_or_manifest(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
