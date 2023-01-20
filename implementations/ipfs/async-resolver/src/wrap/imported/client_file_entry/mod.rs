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
    deserialize_client_file_entry,
    read_client_file_entry,
    serialize_client_file_entry,
    write_client_file_entry
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientFileEntry {
    pub name: String,
    pub data: Vec<u8>,
}

impl ClientFileEntry {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientFileEntry {
        ClientFileEntry {
            name: String::new(),
            data: vec![],
        }
    }

    pub fn to_buffer(args: &ClientFileEntry) -> Result<Vec<u8>, EncodeError> {
        serialize_client_file_entry(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientFileEntry, DecodeError> {
        deserialize_client_file_entry(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientFileEntry, writer: &mut W) -> Result<(), EncodeError> {
        write_client_file_entry(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientFileEntry, DecodeError> {
        read_client_file_entry(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
