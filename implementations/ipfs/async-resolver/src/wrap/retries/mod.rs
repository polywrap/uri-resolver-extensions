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
    deserialize_retries,
    read_retries,
    serialize_retries,
    write_retries
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Retries {
    pub try_resolve_uri: Option<u32>,
    pub get_file: Option<u32>,
}

impl Retries {
    pub fn new() -> Retries {
        Retries {
            try_resolve_uri: None,
            get_file: None,
        }
    }

    pub fn to_buffer(args: &Retries) -> Result<Vec<u8>, EncodeError> {
        serialize_retries(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<Retries, DecodeError> {
        deserialize_retries(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &Retries, writer: &mut W) -> Result<(), EncodeError> {
        write_retries(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Retries, DecodeError> {
        read_retries(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
