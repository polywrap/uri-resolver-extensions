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
    deserialize_client_resolve_options,
    read_client_resolve_options,
    serialize_client_resolve_options,
    write_client_resolve_options
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientResolveOptions {
    pub recursive: Option<bool>,
    pub dht_record_count: Option<i32>,
    pub dht_timeout: Option<String>,
}

impl ClientResolveOptions {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientResolveOptions {
        ClientResolveOptions {
            recursive: None,
            dht_record_count: None,
            dht_timeout: None,
        }
    }

    pub fn to_buffer(args: &ClientResolveOptions) -> Result<Vec<u8>, EncodeError> {
        serialize_client_resolve_options(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientResolveOptions, DecodeError> {
        deserialize_client_resolve_options(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientResolveOptions, writer: &mut W) -> Result<(), EncodeError> {
        write_client_resolve_options(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientResolveOptions, DecodeError> {
        read_client_resolve_options(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
