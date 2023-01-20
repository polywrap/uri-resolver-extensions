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
    deserialize_client_resolve_result,
    read_client_resolve_result,
    serialize_client_resolve_result,
    write_client_resolve_result
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientResolveResult {
    pub cid: String,
    pub provider: String,
}

impl ClientResolveResult {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientResolveResult {
        ClientResolveResult {
            cid: String::new(),
            provider: String::new(),
        }
    }

    pub fn to_buffer(args: &ClientResolveResult) -> Result<Vec<u8>, EncodeError> {
        serialize_client_resolve_result(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientResolveResult, DecodeError> {
        deserialize_client_resolve_result(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientResolveResult, writer: &mut W) -> Result<(), EncodeError> {
        write_client_resolve_result(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientResolveResult, DecodeError> {
        read_client_resolve_result(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
