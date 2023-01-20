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
    deserialize_client_add_options,
    read_client_add_options,
    serialize_client_add_options,
    write_client_add_options
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientAddOptions {
    pub pin: Option<bool>,
    pub only_hash: Option<bool>,
    pub wrap_with_directory: Option<bool>,
}

impl ClientAddOptions {
    pub const URI: &'static str = "wrap://ens/ipfs-http-client.polywrap.eth";

    pub fn new() -> ClientAddOptions {
        ClientAddOptions {
            pin: None,
            only_hash: None,
            wrap_with_directory: None,
        }
    }

    pub fn to_buffer(args: &ClientAddOptions) -> Result<Vec<u8>, EncodeError> {
        serialize_client_add_options(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ClientAddOptions, DecodeError> {
        deserialize_client_add_options(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ClientAddOptions, writer: &mut W) -> Result<(), EncodeError> {
        write_client_add_options(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ClientAddOptions, DecodeError> {
        read_client_add_options(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
