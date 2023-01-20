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
use crate::HttpRequest;

use crate::{
    HttpResponseType,
    get_http_response_type_value,
    sanitize_http_response_type_value
};

pub fn serialize_http_request(args: &HttpRequest) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: HttpRequest".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_http_request(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_http_request<W: Write>(args: &HttpRequest, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&5)?;
    writer.context().push("headers", "Option<Map<String, String>>", "writing property");
    writer.write_string("headers")?;
    writer.write_optional_ext_generic_map(&args.headers, |writer, key| {
        writer.write_string(key)
    }, |writer, value| {
        writer.write_string(value)
    })?;
    writer.context().pop();
    writer.context().push("urlParams", "Option<Map<String, String>>", "writing property");
    writer.write_string("urlParams")?;
    writer.write_optional_ext_generic_map(&args.url_params, |writer, key| {
        writer.write_string(key)
    }, |writer, value| {
        writer.write_string(value)
    })?;
    writer.context().pop();
    writer.context().push("responseType", "HttpResponseType", "writing property");
    writer.write_string("responseType")?;
    writer.write_i32(&(args.response_type as i32))?;
    writer.context().pop();
    writer.context().push("body", "Option<String>", "writing property");
    writer.write_string("body")?;
    writer.write_optional_string(&args.body)?;
    writer.context().pop();
    writer.context().push("timeout", "Option<u32>", "writing property");
    writer.write_string("timeout")?;
    writer.write_optional_u32(&args.timeout)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_http_request(args: &[u8]) -> Result<HttpRequest, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: HttpRequest".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_http_request(&mut reader)
}

pub fn read_http_request<R: Read>(reader: &mut R) -> Result<HttpRequest, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _headers: Option<Map<String, String>> = None;
    let mut _url_params: Option<Map<String, String>> = None;
    let mut _response_type: HttpResponseType = HttpResponseType::_MAX_;
    let mut _response_type_set = false;
    let mut _body: Option<String> = None;
    let mut _timeout: Option<u32> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "headers" => {
                reader.context().push(&field, "Option<Map<String, String>>", "type found, reading property");
                _headers = reader.read_optional_ext_generic_map(|reader| {
                    reader.read_string()
                }, |reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "urlParams" => {
                reader.context().push(&field, "Option<Map<String, String>>", "type found, reading property");
                _url_params = reader.read_optional_ext_generic_map(|reader| {
                    reader.read_string()
                }, |reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "responseType" => {
                reader.context().push(&field, "HttpResponseType", "type found, reading property");
                let mut value: HttpResponseType = HttpResponseType::_MAX_;
                if reader.is_next_string()? {
                    value = get_http_response_type_value(&reader.read_string()?)?;
                } else {
                    value = HttpResponseType::try_from(reader.read_i32()?)?;
                    sanitize_http_response_type_value(value as i32)?;
                }
                _response_type = value;
                _response_type_set = true;
                reader.context().pop();
            }
            "body" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _body = reader.read_optional_string()?;
                reader.context().pop();
            }
            "timeout" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _timeout = reader.read_optional_u32()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_response_type_set {
        return Err(DecodeError::MissingField("responseType: Http_ResponseType.".to_string()));
    }

    Ok(HttpRequest {
        headers: _headers,
        url_params: _url_params,
        response_type: _response_type,
        body: _body,
        timeout: _timeout,
    })
}
