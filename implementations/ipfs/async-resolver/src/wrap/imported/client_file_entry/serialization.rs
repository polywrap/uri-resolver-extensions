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
use crate::ClientFileEntry;

pub fn serialize_client_file_entry(args: &ClientFileEntry) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ClientFileEntry".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_client_file_entry(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_client_file_entry<W: Write>(args: &ClientFileEntry, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("name", "String", "writing property");
    writer.write_string("name")?;
    writer.write_string(&args.name)?;
    writer.context().pop();
    writer.context().push("data", "Vec<u8>", "writing property");
    writer.write_string("data")?;
    writer.write_bytes(&args.data)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_client_file_entry(args: &[u8]) -> Result<ClientFileEntry, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ClientFileEntry".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_client_file_entry(&mut reader)
}

pub fn read_client_file_entry<R: Read>(reader: &mut R) -> Result<ClientFileEntry, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _name: String = String::new();
    let mut _name_set = false;
    let mut _data: Vec<u8> = vec![];
    let mut _data_set = false;

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
            "data" => {
                reader.context().push(&field, "Vec<u8>", "type found, reading property");
                _data = reader.read_bytes()?;
                _data_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_name_set {
        return Err(DecodeError::MissingField("name: String.".to_string()));
    }
    if !_data_set {
        return Err(DecodeError::MissingField("data: Bytes.".to_string()));
    }

    Ok(ClientFileEntry {
        name: _name,
        data: _data,
    })
}
