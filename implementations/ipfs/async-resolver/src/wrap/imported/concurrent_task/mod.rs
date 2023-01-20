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
    deserialize_concurrent_task,
    read_concurrent_task,
    serialize_concurrent_task,
    write_concurrent_task
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConcurrentTask {
    pub uri: String,
    pub method: String,
    pub args: Vec<u8>,
}

impl ConcurrentTask {
    pub const URI: &'static str = "wrap://ens/goerli/interface.concurrent.wrappers.eth";

    pub fn new() -> ConcurrentTask {
        ConcurrentTask {
            uri: String::new(),
            method: String::new(),
            args: vec![],
        }
    }

    pub fn to_buffer(args: &ConcurrentTask) -> Result<Vec<u8>, EncodeError> {
        serialize_concurrent_task(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ConcurrentTask, DecodeError> {
        deserialize_concurrent_task(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ConcurrentTask, writer: &mut W) -> Result<(), EncodeError> {
        write_concurrent_task(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ConcurrentTask, DecodeError> {
        read_concurrent_task(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
