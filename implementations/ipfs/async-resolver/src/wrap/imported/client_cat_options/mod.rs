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
    deserialize_client_cat_options,
    read_client_cat_options,
    serialize_client_cat_options,
    write_client_cat_options
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientCatOptions {
    pub offset: Option<i32>,
    pub length: Option<i32>,
}

impl ClientCatOptions {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientCatOptions {
        ClientCatOptions {
            offset: None,
            length: None,
        }
    }

    pub fn to_buffer(args: &ClientCatOptions) -> Result<Vec<u8>, EncodeError> {
        serialize_client_cat_options(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientCatOptions, DecodeError> {
        deserialize_client_cat_options(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientCatOptions, writer: &mut W) -> Result<(), EncodeError> {
        write_client_cat_options(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientCatOptions, DecodeError> {
        read_client_cat_options(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
