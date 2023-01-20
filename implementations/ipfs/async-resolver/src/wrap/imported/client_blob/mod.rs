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
    deserialize_client_blob,
    read_client_blob,
    serialize_client_blob,
    write_client_blob
};

use crate::ClientDirectoryEntry;
use crate::ClientFileEntry;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientBlob {
    pub directories: Option<Vec<ClientDirectoryEntry>>,
    pub files: Option<Vec<ClientFileEntry>>,
}

impl ClientBlob {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientBlob {
        ClientBlob {
            directories: None,
            files: None,
        }
    }

    pub fn to_buffer(args: &ClientBlob) -> Result<Vec<u8>, EncodeError> {
        serialize_client_blob(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientBlob, DecodeError> {
        deserialize_client_blob(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientBlob, writer: &mut W) -> Result<(), EncodeError> {
        write_client_blob(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientBlob, DecodeError> {
        read_client_blob(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
