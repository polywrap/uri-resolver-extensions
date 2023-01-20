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
    deserialize_client_directory_entry,
    read_client_directory_entry,
    serialize_client_directory_entry,
    write_client_directory_entry
};

use crate::ClientFileEntry;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientDirectoryEntry {
    pub name: String,
    pub directories: Option<Vec<ClientDirectoryEntry>>,
    pub files: Option<Vec<ClientFileEntry>>,
}

impl ClientDirectoryEntry {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientDirectoryEntry {
        ClientDirectoryEntry {
            name: String::new(),
            directories: None,
            files: None,
        }
    }

    pub fn to_buffer(args: &ClientDirectoryEntry) -> Result<Vec<u8>, EncodeError> {
        serialize_client_directory_entry(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientDirectoryEntry, DecodeError> {
        deserialize_client_directory_entry(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientDirectoryEntry, writer: &mut W) -> Result<(), EncodeError> {
        write_client_directory_entry(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientDirectoryEntry, DecodeError> {
        read_client_directory_entry(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
