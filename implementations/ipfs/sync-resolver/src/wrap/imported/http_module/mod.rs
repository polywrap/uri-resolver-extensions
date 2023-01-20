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
    deserialize_get_result,
    serialize_get_args,
    ArgsGet,
    deserialize_post_result,
    serialize_post_args,
    ArgsPost
};

use crate::HttpRequest;
use crate::HttpResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpModule {}

impl HttpModule {
    pub const URI: &'static str = "wrap://ens/http.polywrap.eth";

    pub fn new() -> HttpModule {
        HttpModule {}
    }

    pub fn get(args: &ArgsGet) -> Result<Option<HttpResponse>, String> {
        let uri = HttpModule::URI;
        let args = serialize_get_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "get",
            args,
        )?;
        deserialize_get_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn post(args: &ArgsPost) -> Result<Option<HttpResponse>, String> {
        let uri = HttpModule::URI;
        let args = serialize_post_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "post",
            args,
        )?;
        deserialize_post_result(result.as_slice()).map_err(|e| e.to_string())
    }
}
