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
    deserialize_http_request,
    read_http_request,
    serialize_http_request,
    write_http_request
};

use crate::HttpResponseType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpRequest {
    pub headers: Option<Map<String, String>>,
    pub url_params: Option<Map<String, String>>,
    pub response_type: HttpResponseType,
    pub body: Option<String>,
    pub timeout: Option<u32>,
}

impl HttpRequest {
    pub const URI: &'static str = "wrap://ens/http.polywrap.eth";

    pub fn new() -> HttpRequest {
        HttpRequest {
            headers: None,
            url_params: None,
            response_type: HttpResponseType::_MAX_,
            body: None,
            timeout: None,
        }
    }

    pub fn to_buffer(args: &HttpRequest) -> Result<Vec<u8>, EncodeError> {
        serialize_http_request(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<HttpRequest, DecodeError> {
        deserialize_http_request(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &HttpRequest, writer: &mut W) -> Result<(), EncodeError> {
        write_http_request(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<HttpRequest, DecodeError> {
        read_http_request(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
