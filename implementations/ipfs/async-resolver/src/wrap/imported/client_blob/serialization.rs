use std::convert::TryFrom;
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
use crate::ClientBlob;

use crate::ClientDirectoryEntry;
use crate::ClientFileEntry;

pub fn serialize_client_blob(args: &ClientBlob) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientBlob".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_blob(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_blob<W: Write>(args: &ClientBlob, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("directories", "Option<Vec<ClientDirectoryEntry>>", "writing property");
    writer.write_string("directories")?;
    writer.write_optional_array(&args.directories, |writer, item| {
        ClientDirectoryEntry::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("files", "Option<Vec<ClientFileEntry>>", "writing property");
    writer.write_string("files")?;
    writer.write_optional_array(&args.files, |writer, item| {
        ClientFileEntry::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_client_blob(args: &[u8]) -> Result<ClientBlob, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientBlob".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_blob(&mut reader)
}

pub fn read_client_blob<R: Read>(reader: &mut R) -> Result<ClientBlob, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _directories: Option<Vec<ClientDirectoryEntry>> = None;
    let mut _files: Option<Vec<ClientFileEntry>> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "directories" => {
                reader.context().push(&field, "Option<Vec<ClientDirectoryEntry>>", "type found, reading property");
                _directories = reader.read_optional_array(|reader| {
                    let object = ClientDirectoryEntry::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "files" => {
                reader.context().push(&field, "Option<Vec<ClientFileEntry>>", "type found, reading property");
                _files = reader.read_optional_array(|reader| {
                    let object = ClientFileEntry::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ClientBlob {
        directories: _directories,
        files: _files,
    })
}
