use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
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
pub struct ArgsCat {
    pub cid: String,
    pub ipfs_provider: String,
    pub timeout: Option<u32>,
    pub cat_options: Option<ClientCatOptions>,
}

pub fn deserialize_cat_args(args: &[u8]) -> Result<ArgsCat, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: cat Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _cid: String = String::new();
    let mut _cid_set = false;
    let mut _ipfs_provider: String = String::new();
    let mut _ipfs_provider_set = false;
    let mut _timeout: Option<u32> = None;
    let mut _cat_options: Option<ClientCatOptions> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "cid" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _cid = reader.read_string()?;
                _cid_set = true;
                reader.context().pop();
            }
            "ipfsProvider" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _ipfs_provider = reader.read_string()?;
                _ipfs_provider_set = true;
                reader.context().pop();
            }
            "timeout" => {
                reader.context().push(&field, "Option<u32>", "type found, reading argument");
                _timeout = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "catOptions" => {
                reader.context().push(&field, "Option<ClientCatOptions>", "type found, reading argument");
                let mut object: Option<ClientCatOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(ClientCatOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _cat_options = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_cid_set {
        return Err(DecodeError::MissingField("cid: String.".to_string()));
    }
    if !_ipfs_provider_set {
        return Err(DecodeError::MissingField("ipfsProvider: String.".to_string()));
    }

    Ok(ArgsCat {
        cid: _cid,
        ipfs_provider: _ipfs_provider,
        timeout: _timeout,
        cat_options: _cat_options,
    })
}

pub fn serialize_cat_args(args: &ArgsCat) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: cat Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_cat_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_cat_args<W: Write>(args: &ArgsCat, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("cid", "String", "writing property");
    writer.write_string("cid")?;
    writer.write_string(&args.cid)?;
    writer.context().pop();
    writer.context().push("ipfsProvider", "String", "writing property");
    writer.write_string("ipfsProvider")?;
    writer.write_string(&args.ipfs_provider)?;
    writer.context().pop();
    writer.context().push("timeout", "Option<u32>", "writing property");
    writer.write_string("timeout")?;
    writer.write_optional_u32(&args.timeout)?;
    writer.context().pop();
    writer.context().push("catOptions", "Option<ClientCatOptions>", "writing property");
    writer.write_string("catOptions")?;
    if args.cat_options.is_some() {
        ClientCatOptions::write(args.cat_options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_cat_result(result: &Vec<u8>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: cat Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_cat_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_cat_result<W: Write>(result: &Vec<u8>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("cat", "Vec<u8>", "writing result");
    writer.write_bytes(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_cat_result(result: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: cat Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("cat", "Vec<u8>", "reading function output");
    let res = reader.read_bytes()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsResolve {
    pub cid: String,
    pub ipfs_provider: String,
    pub timeout: Option<u32>,
    pub resolve_options: Option<ClientResolveOptions>,
}

pub fn deserialize_resolve_args(args: &[u8]) -> Result<ArgsResolve, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: resolve Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _cid: String = String::new();
    let mut _cid_set = false;
    let mut _ipfs_provider: String = String::new();
    let mut _ipfs_provider_set = false;
    let mut _timeout: Option<u32> = None;
    let mut _resolve_options: Option<ClientResolveOptions> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "cid" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _cid = reader.read_string()?;
                _cid_set = true;
                reader.context().pop();
            }
            "ipfsProvider" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _ipfs_provider = reader.read_string()?;
                _ipfs_provider_set = true;
                reader.context().pop();
            }
            "timeout" => {
                reader.context().push(&field, "Option<u32>", "type found, reading argument");
                _timeout = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "resolveOptions" => {
                reader.context().push(&field, "Option<ClientResolveOptions>", "type found, reading argument");
                let mut object: Option<ClientResolveOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(ClientResolveOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _resolve_options = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_cid_set {
        return Err(DecodeError::MissingField("cid: String.".to_string()));
    }
    if !_ipfs_provider_set {
        return Err(DecodeError::MissingField("ipfsProvider: String.".to_string()));
    }

    Ok(ArgsResolve {
        cid: _cid,
        ipfs_provider: _ipfs_provider,
        timeout: _timeout,
        resolve_options: _resolve_options,
    })
}

pub fn serialize_resolve_args(args: &ArgsResolve) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: resolve Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_resolve_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_resolve_args<W: Write>(args: &ArgsResolve, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("cid", "String", "writing property");
    writer.write_string("cid")?;
    writer.write_string(&args.cid)?;
    writer.context().pop();
    writer.context().push("ipfsProvider", "String", "writing property");
    writer.write_string("ipfsProvider")?;
    writer.write_string(&args.ipfs_provider)?;
    writer.context().pop();
    writer.context().push("timeout", "Option<u32>", "writing property");
    writer.write_string("timeout")?;
    writer.write_optional_u32(&args.timeout)?;
    writer.context().pop();
    writer.context().push("resolveOptions", "Option<ClientResolveOptions>", "writing property");
    writer.write_string("resolveOptions")?;
    if args.resolve_options.is_some() {
        ClientResolveOptions::write(args.resolve_options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_resolve_result(result: &ClientResolveResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: resolve Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_resolve_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_resolve_result<W: Write>(result: &ClientResolveResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("resolve", "ClientResolveResult", "writing result");
    ClientResolveResult::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_resolve_result(result: &[u8]) -> Result<ClientResolveResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: resolve Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("resolve", "ClientResolveResult", "reading function output");
    let object = ClientResolveResult::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsAddFile {
    pub data: ClientFileEntry,
    pub ipfs_provider: String,
    pub timeout: Option<u32>,
    pub add_options: Option<ClientAddOptions>,
}

pub fn deserialize_add_file_args(args: &[u8]) -> Result<ArgsAddFile, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: add_file Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _data: ClientFileEntry = ClientFileEntry::new();
    let mut _data_set = false;
    let mut _ipfs_provider: String = String::new();
    let mut _ipfs_provider_set = false;
    let mut _timeout: Option<u32> = None;
    let mut _add_options: Option<ClientAddOptions> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "data" => {
                reader.context().push(&field, "ClientFileEntry", "type found, reading argument");
                let object = ClientFileEntry::read(&mut reader)?;
                _data = object;
                _data_set = true;
                reader.context().pop();
            }
            "ipfsProvider" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _ipfs_provider = reader.read_string()?;
                _ipfs_provider_set = true;
                reader.context().pop();
            }
            "timeout" => {
                reader.context().push(&field, "Option<u32>", "type found, reading argument");
                _timeout = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "addOptions" => {
                reader.context().push(&field, "Option<ClientAddOptions>", "type found, reading argument");
                let mut object: Option<ClientAddOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(ClientAddOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _add_options = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_data_set {
        return Err(DecodeError::MissingField("data: Client_FileEntry.".to_string()));
    }
    if !_ipfs_provider_set {
        return Err(DecodeError::MissingField("ipfsProvider: String.".to_string()));
    }

    Ok(ArgsAddFile {
        data: _data,
        ipfs_provider: _ipfs_provider,
        timeout: _timeout,
        add_options: _add_options,
    })
}

pub fn serialize_add_file_args(args: &ArgsAddFile) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: add_file Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_add_file_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_add_file_args<W: Write>(args: &ArgsAddFile, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("data", "ClientFileEntry", "writing property");
    writer.write_string("data")?;
    ClientFileEntry::write(&args.data, writer)?;
    writer.context().pop();
    writer.context().push("ipfsProvider", "String", "writing property");
    writer.write_string("ipfsProvider")?;
    writer.write_string(&args.ipfs_provider)?;
    writer.context().pop();
    writer.context().push("timeout", "Option<u32>", "writing property");
    writer.write_string("timeout")?;
    writer.write_optional_u32(&args.timeout)?;
    writer.context().pop();
    writer.context().push("addOptions", "Option<ClientAddOptions>", "writing property");
    writer.write_string("addOptions")?;
    if args.add_options.is_some() {
        ClientAddOptions::write(args.add_options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_add_file_result(result: &ClientAddResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: add_file Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_add_file_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_add_file_result<W: Write>(result: &ClientAddResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("addFile", "ClientAddResult", "writing result");
    ClientAddResult::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_add_file_result(result: &[u8]) -> Result<ClientAddResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: add_file Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("addFile", "ClientAddResult", "reading function output");
    let object = ClientAddResult::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsAddDir {
    pub data: ClientDirectoryEntry,
    pub ipfs_provider: String,
    pub timeout: Option<u32>,
    pub add_options: Option<ClientAddOptions>,
}

pub fn deserialize_add_dir_args(args: &[u8]) -> Result<ArgsAddDir, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: add_dir Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _data: ClientDirectoryEntry = ClientDirectoryEntry::new();
    let mut _data_set = false;
    let mut _ipfs_provider: String = String::new();
    let mut _ipfs_provider_set = false;
    let mut _timeout: Option<u32> = None;
    let mut _add_options: Option<ClientAddOptions> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "data" => {
                reader.context().push(&field, "ClientDirectoryEntry", "type found, reading argument");
                let object = ClientDirectoryEntry::read(&mut reader)?;
                _data = object;
                _data_set = true;
                reader.context().pop();
            }
            "ipfsProvider" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _ipfs_provider = reader.read_string()?;
                _ipfs_provider_set = true;
                reader.context().pop();
            }
            "timeout" => {
                reader.context().push(&field, "Option<u32>", "type found, reading argument");
                _timeout = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "addOptions" => {
                reader.context().push(&field, "Option<ClientAddOptions>", "type found, reading argument");
                let mut object: Option<ClientAddOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(ClientAddOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _add_options = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_data_set {
        return Err(DecodeError::MissingField("data: Client_DirectoryEntry.".to_string()));
    }
    if !_ipfs_provider_set {
        return Err(DecodeError::MissingField("ipfsProvider: String.".to_string()));
    }

    Ok(ArgsAddDir {
        data: _data,
        ipfs_provider: _ipfs_provider,
        timeout: _timeout,
        add_options: _add_options,
    })
}

pub fn serialize_add_dir_args(args: &ArgsAddDir) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: add_dir Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_add_dir_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_add_dir_args<W: Write>(args: &ArgsAddDir, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("data", "ClientDirectoryEntry", "writing property");
    writer.write_string("data")?;
    ClientDirectoryEntry::write(&args.data, writer)?;
    writer.context().pop();
    writer.context().push("ipfsProvider", "String", "writing property");
    writer.write_string("ipfsProvider")?;
    writer.write_string(&args.ipfs_provider)?;
    writer.context().pop();
    writer.context().push("timeout", "Option<u32>", "writing property");
    writer.write_string("timeout")?;
    writer.write_optional_u32(&args.timeout)?;
    writer.context().pop();
    writer.context().push("addOptions", "Option<ClientAddOptions>", "writing property");
    writer.write_string("addOptions")?;
    if args.add_options.is_some() {
        ClientAddOptions::write(args.add_options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_add_dir_result(result: &Vec<ClientAddResult>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: add_dir Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_add_dir_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_add_dir_result<W: Write>(result: &Vec<ClientAddResult>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("addDir", "Vec<ClientAddResult>", "writing result");
    writer.write_array(&result, |writer, item| {
        ClientAddResult::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_add_dir_result(result: &[u8]) -> Result<Vec<ClientAddResult>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: add_dir Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("addDir", "Vec<ClientAddResult>", "reading function output");
    let res = reader.read_array(|reader| {
        let object = ClientAddResult::read(reader)?;
        Ok(object)
    })?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsAddBlob {
    pub data: ClientBlob,
    pub ipfs_provider: String,
    pub timeout: Option<u32>,
    pub add_options: Option<ClientAddOptions>,
}

pub fn deserialize_add_blob_args(args: &[u8]) -> Result<ArgsAddBlob, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: add_blob Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _data: ClientBlob = ClientBlob::new();
    let mut _data_set = false;
    let mut _ipfs_provider: String = String::new();
    let mut _ipfs_provider_set = false;
    let mut _timeout: Option<u32> = None;
    let mut _add_options: Option<ClientAddOptions> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "data" => {
                reader.context().push(&field, "ClientBlob", "type found, reading argument");
                let object = ClientBlob::read(&mut reader)?;
                _data = object;
                _data_set = true;
                reader.context().pop();
            }
            "ipfsProvider" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _ipfs_provider = reader.read_string()?;
                _ipfs_provider_set = true;
                reader.context().pop();
            }
            "timeout" => {
                reader.context().push(&field, "Option<u32>", "type found, reading argument");
                _timeout = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "addOptions" => {
                reader.context().push(&field, "Option<ClientAddOptions>", "type found, reading argument");
                let mut object: Option<ClientAddOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(ClientAddOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _add_options = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_data_set {
        return Err(DecodeError::MissingField("data: Client_Blob.".to_string()));
    }
    if !_ipfs_provider_set {
        return Err(DecodeError::MissingField("ipfsProvider: String.".to_string()));
    }

    Ok(ArgsAddBlob {
        data: _data,
        ipfs_provider: _ipfs_provider,
        timeout: _timeout,
        add_options: _add_options,
    })
}

pub fn serialize_add_blob_args(args: &ArgsAddBlob) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: add_blob Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_add_blob_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_add_blob_args<W: Write>(args: &ArgsAddBlob, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("data", "ClientBlob", "writing property");
    writer.write_string("data")?;
    ClientBlob::write(&args.data, writer)?;
    writer.context().pop();
    writer.context().push("ipfsProvider", "String", "writing property");
    writer.write_string("ipfsProvider")?;
    writer.write_string(&args.ipfs_provider)?;
    writer.context().pop();
    writer.context().push("timeout", "Option<u32>", "writing property");
    writer.write_string("timeout")?;
    writer.write_optional_u32(&args.timeout)?;
    writer.context().pop();
    writer.context().push("addOptions", "Option<ClientAddOptions>", "writing property");
    writer.write_string("addOptions")?;
    if args.add_options.is_some() {
        ClientAddOptions::write(args.add_options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_add_blob_result(result: &Vec<ClientAddResult>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: add_blob Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_add_blob_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_add_blob_result<W: Write>(result: &Vec<ClientAddResult>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("addBlob", "Vec<ClientAddResult>", "writing result");
    writer.write_array(&result, |writer, item| {
        ClientAddResult::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_add_blob_result(result: &[u8]) -> Result<Vec<ClientAddResult>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: add_blob Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("addBlob", "Vec<ClientAddResult>", "reading function output");
    let res = reader.read_array(|reader| {
        let object = ClientAddResult::read(reader)?;
        Ok(object)
    })?;
    reader.context().pop();
    Ok(res)
}
