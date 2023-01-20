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
    deserialize_client_add_result,
    read_client_add_result,
    serialize_client_add_result,
    write_client_add_result
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientAddResult {
    pub name: String,
    pub hash: String,
    pub size: String,
}

impl ClientAddResult {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientAddResult {
        ClientAddResult {
            name: String::new(),
            hash: String::new(),
            size: String::new(),
        }
    }

    pub fn to_buffer(args: &ClientAddResult) -> Result<Vec<u8>, EncodeError> {
        serialize_client_add_result(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientAddResult, DecodeError> {
        deserialize_client_add_result(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientAddResult, writer: &mut W) -> Result<(), EncodeError> {
        write_client_add_result(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientAddResult, DecodeError> {
        read_client_add_result(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
