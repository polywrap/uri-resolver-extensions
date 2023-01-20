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
use crate::HttpResponse;

pub fn serialize_http_response(args: &HttpResponse) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: HttpResponse".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_http_response(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_http_response<W: Write>(args: &HttpResponse, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("status", "i32", "writing property");
    writer.write_string("status")?;
    writer.write_i32(&args.status)?;
    writer.context().pop();
    writer.context().push("statusText", "String", "writing property");
    writer.write_string("statusText")?;
    writer.write_string(&args.status_text)?;
    writer.context().pop();
    writer.context().push("headers", "Option<Map<String, String>>", "writing property");
    writer.write_string("headers")?;
    writer.write_optional_ext_generic_map(&args.headers, |writer, key| {
        writer.write_string(key)
    }, |writer, value| {
        writer.write_string(value)
    })?;
    writer.context().pop();
    writer.context().push("body", "Option<String>", "writing property");
    writer.write_string("body")?;
    writer.write_optional_string(&args.body)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_http_response(args: &[u8]) -> Result<HttpResponse, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: HttpResponse".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_http_response(&mut reader)
}

pub fn read_http_response<R: Read>(reader: &mut R) -> Result<HttpResponse, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _status: i32 = 0;
    let mut _status_set = false;
    let mut _status_text: String = String::new();
    let mut _status_text_set = false;
    let mut _headers: Option<Map<String, String>> = None;
    let mut _body: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "status" => {
                reader.context().push(&field, "i32", "type found, reading property");
                _status = reader.read_i32()?;
                _status_set = true;
                reader.context().pop();
            }
            "statusText" => {
                reader.context().push(&field, "String", "type found, reading property");
                _status_text = reader.read_string()?;
                _status_text_set = true;
                reader.context().pop();
            }
            "headers" => {
                reader.context().push(&field, "Option<Map<String, String>>", "type found, reading property");
                _headers = reader.read_optional_ext_generic_map(|reader| {
                    reader.read_string()
                }, |reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "body" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _body = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_status_set {
        return Err(DecodeError::MissingField("status: Int.".to_string()));
    }
    if !_status_text_set {
        return Err(DecodeError::MissingField("statusText: String.".to_string()));
    }

    Ok(HttpResponse {
        status: _status,
        status_text: _status_text,
        headers: _headers,
        body: _body,
    })
}
