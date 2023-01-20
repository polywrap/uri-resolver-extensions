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
    deserialize_try_resolve_uri_result,
    serialize_try_resolve_uri_args,
    ArgsTryResolveUri,
    deserialize_get_file_result,
    serialize_get_file_args,
    ArgsGetFile
};

use crate::UriResolverMaybeUriOrManifest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UriResolverModule {}

impl UriResolverModule {
    pub const URI: &'static str = "ens/interface.resolver.polywrap.eth";

    pub fn new() -> UriResolverModule {
        UriResolverModule {}
    }

    pub fn try_resolve_uri(args: &ArgsTryResolveUri) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        let uri = UriResolverModule::URI;
        let args = serialize_try_resolve_uri_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "tryResolveUri",
            args,
        )?;
        deserialize_try_resolve_uri_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn get_file(args: &ArgsGetFile) -> Result<Option<Vec<u8>>, String> {
        let uri = UriResolverModule::URI;
        let args = serialize_get_file_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "getFile",
            args,
        )?;
        deserialize_get_file_result(result.as_slice()).map_err(|e| e.to_string())
    }
}
