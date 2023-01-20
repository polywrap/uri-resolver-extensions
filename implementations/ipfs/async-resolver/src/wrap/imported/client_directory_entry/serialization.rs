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
use crate::ClientDirectoryEntry;

use crate::ClientFileEntry;

pub fn serialize_client_directory_entry(args: &ClientDirectoryEntry) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientDirectoryEntry".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_directory_entry(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_directory_entry<W: Write>(args: &ClientDirectoryEntry, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("name", "String", "writing property");
    writer.write_string("name")?;
    writer.write_string(&args.name)?;
    writer.context().pop();
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

pub fn deserialize_client_directory_entry(args: &[u8]) -> Result<ClientDirectoryEntry, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientDirectoryEntry".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_directory_entry(&mut reader)
}

pub fn read_client_directory_entry<R: Read>(reader: &mut R) -> Result<ClientDirectoryEntry, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _name: String = String::new();
    let mut _name_set = false;
    let mut _directories: Option<Vec<ClientDirectoryEntry>> = None;
    let mut _files: Option<Vec<ClientFileEntry>> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "name" => {
                reader.context().push(&field, "String", "type found, reading property");
                _name = reader.read_string()?;
                _name_set = true;
                reader.context().pop();
            }
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
    if !_name_set {
        return Err(DecodeError::MissingField("name: String.".to_string()));
    }

    Ok(ClientDirectoryEntry {
        name: _name,
        directories: _directories,
        files: _files,
    })
}
