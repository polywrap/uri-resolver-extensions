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
    deserialize_cat_result,
    serialize_cat_args,
    ArgsCat,
    deserialize_resolve_result,
    serialize_resolve_args,
    ArgsResolve,
    deserialize_add_file_result,
    serialize_add_file_args,
    ArgsAddFile,
    deserialize_add_dir_result,
    serialize_add_dir_args,
    ArgsAddDir,
    deserialize_add_blob_result,
    serialize_add_blob_args,
    ArgsAddBlob
};

use crate::ClientCatOptions;
use crate::ClientResolveOptions;
use crate::ClientResolveResult;
use crate::ClientFileEntry;
use crate::ClientAddOptions;
use crate::ClientAddResult;
use crate::ClientDirectoryEntry;
use crate::ClientBlob;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientModule {}

impl ClientModule {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientModule {
        ClientModule {}
    }

    pub fn cat(args: &ArgsCat) -> Result<Vec<u8>, String> {
        let uri = ClientModule::URI;
        let args = serialize_cat_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "cat",
            args,
        )?;
        deserialize_cat_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn resolve(args: &ArgsResolve) -> Result<ClientResolveResult, String> {
        let uri = ClientModule::URI;
        let args = serialize_resolve_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "resolve",
            args,
        )?;
        deserialize_resolve_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn add_file(args: &ArgsAddFile) -> Result<ClientAddResult, String> {
        let uri = ClientModule::URI;
        let args = serialize_add_file_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "addFile",
            args,
        )?;
        deserialize_add_file_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn add_dir(args: &ArgsAddDir) -> Result<Vec<ClientAddResult>, String> {
        let uri = ClientModule::URI;
        let args = serialize_add_dir_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "addDir",
            args,
        )?;
        deserialize_add_dir_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn add_blob(args: &ArgsAddBlob) -> Result<Vec<ClientAddResult>, String> {
        let uri = ClientModule::URI;
        let args = serialize_add_blob_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "addBlob",
            args,
        )?;
        deserialize_add_blob_result(result.as_slice()).map_err(|e| e.to_string())
    }
}
