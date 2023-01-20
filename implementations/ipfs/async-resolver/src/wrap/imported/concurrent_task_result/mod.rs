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
    deserialize_concurrent_task_result,
    read_concurrent_task_result,
    serialize_concurrent_task_result,
    write_concurrent_task_result
};

use crate::ConcurrentTaskStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConcurrentTaskResult {
    pub task_id: i32,
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
    pub status: ConcurrentTaskStatus,
}

impl ConcurrentTaskResult {
    pub const URI: &'static str = "wrap://ens/goerli/interface.concurrent.wrappers.eth";

    pub fn new() -> ConcurrentTaskResult {
        ConcurrentTaskResult {
            task_id: 0,
            result: None,
            error: None,
            status: ConcurrentTaskStatus::_MAX_,
        }
    }

    pub fn to_buffer(args: &ConcurrentTaskResult) -> Result<Vec<u8>, EncodeError> {
        serialize_concurrent_task_result(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ConcurrentTaskResult, DecodeError> {
        deserialize_concurrent_task_result(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ConcurrentTaskResult, writer: &mut W) -> Result<(), EncodeError> {
        write_concurrent_task_result(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ConcurrentTaskResult, DecodeError> {
        read_concurrent_task_result(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
