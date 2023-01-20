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
    deserialize_http_response,
    read_http_response,
    serialize_http_response,
    write_http_response
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpResponse {
    pub status: i32,
    pub status_text: String,
    pub headers: Option<Map<String, String>>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub const URI: &'static str = "wrap://ens/http.polywrap.eth";

    pub fn new() -> HttpResponse {
        HttpResponse {
            status: 0,
            status_text: String::new(),
            headers: None,
            body: None,
        }
    }

    pub fn to_buffer(args: &HttpResponse) -> Result<Vec<u8>, EncodeError> {
        serialize_http_response(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<HttpResponse, DecodeError> {
        deserialize_http_response(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &HttpResponse, writer: &mut W) -> Result<(), EncodeError> {
        write_http_response(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<HttpResponse, DecodeError> {
        read_http_response(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
