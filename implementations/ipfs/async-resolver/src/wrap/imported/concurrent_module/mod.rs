use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Read,
    Write,
    JSON,
    subinvoke,
};
pub mod serialization;
pub use serialization::{
    deserialize_result_result,
    serialize_result_args,
    ArgsResult,
    deserialize_status_result,
    serialize_status_args,
    ArgsStatus,
    deserialize_schedule_result,
    serialize_schedule_args,
    ArgsSchedule,
    deserialize_abort_result,
    serialize_abort_args,
    ArgsAbort
};

use crate::ConcurrentReturnWhen;
use crate::ConcurrentTaskResult;
use crate::ConcurrentTaskStatus;
use crate::ConcurrentTask;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConcurrentModule {
    uri: String
}

impl ConcurrentModule {
    pub const INTERFACE_URI: &'static str = "wrap://ens/goerli/interface.concurrent.wrappers.eth";

    pub fn new(uri: String) -> ConcurrentModule {
        ConcurrentModule { uri }
    }

    pub fn result(&self, args: &ArgsResult) -> Result<Vec<ConcurrentTaskResult>, String> {
        let ref uri = self.uri;
        let args = serialize_result_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri.as_str(),
            "result",
            args,
        )?;
        deserialize_result_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn status(&self, args: &ArgsStatus) -> Result<Vec<ConcurrentTaskStatus>, String> {
        let ref uri = self.uri;
        let args = serialize_status_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri.as_str(),
            "status",
            args,
        )?;
        deserialize_status_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn schedule(&self, args: &ArgsSchedule) -> Result<Vec<i32>, String> {
        let ref uri = self.uri;
        let args = serialize_schedule_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri.as_str(),
            "schedule",
            args,
        )?;
        deserialize_schedule_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn abort(&self, args: &ArgsAbort) -> Result<Vec<bool>, String> {
        let ref uri = self.uri;
        let args = serialize_abort_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri.as_str(),
            "abort",
            args,
        )?;
        deserialize_abort_result(result.as_slice()).map_err(|e| e.to_string())
    }
}
