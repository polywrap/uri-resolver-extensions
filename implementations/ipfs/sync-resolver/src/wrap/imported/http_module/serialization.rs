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

use crate::HttpRequest;
use crate::HttpResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGet {
    pub url: String,
    pub request: Option<HttpRequest>,
}

pub fn deserialize_get_args(args: &[u8]) -> Result<ArgsGet, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: get Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _url: String = String::new();
    let mut _url_set = false;
    let mut _request: Option<HttpRequest> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "url" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _url = reader.read_string()?;
                _url_set = true;
                reader.context().pop();
            }
            "request" => {
                reader.context().push(&field, "Option<HttpRequest>", "type found, reading argument");
                let mut object: Option<HttpRequest> = None;
                if !reader.is_next_nil()? {
                    object = Some(HttpRequest::read(&mut reader)?);
                } else {
                    object = None;
                }
                _request = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_url_set {
        return Err(DecodeError::MissingField("url: String.".to_string()));
    }

    Ok(ArgsGet {
        url: _url,
        request: _request,
    })
}

pub fn serialize_get_args(args: &ArgsGet) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: get Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_args<W: Write>(args: &ArgsGet, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("url", "String", "writing property");
    writer.write_string("url")?;
    writer.write_string(&args.url)?;
    writer.context().pop();
    writer.context().push("request", "Option<HttpRequest>", "writing property");
    writer.write_string("request")?;
    if args.request.is_some() {
        HttpRequest::write(args.request.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_result(result: &Option<HttpResponse>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: get Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_result<W: Write>(result: &Option<HttpResponse>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("get", "Option<HttpResponse>", "writing result");
    if result.is_some() {
        HttpResponse::write(result.as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_result(result: &[u8]) -> Result<Option<HttpResponse>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: get Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("get", "Option<HttpResponse>", "reading function output");
    let mut object: Option<HttpResponse> = None;
    if !reader.is_next_nil()? {
        object = Some(HttpResponse::read(&mut reader)?);
    } else {
        object = None;
    }
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsPost {
    pub url: String,
    pub request: Option<HttpRequest>,
}

pub fn deserialize_post_args(args: &[u8]) -> Result<ArgsPost, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: post Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _url: String = String::new();
    let mut _url_set = false;
    let mut _request: Option<HttpRequest> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "url" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _url = reader.read_string()?;
                _url_set = true;
                reader.context().pop();
            }
            "request" => {
                reader.context().push(&field, "Option<HttpRequest>", "type found, reading argument");
                let mut object: Option<HttpRequest> = None;
                if !reader.is_next_nil()? {
                    object = Some(HttpRequest::read(&mut reader)?);
                } else {
                    object = None;
                }
                _request = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_url_set {
        return Err(DecodeError::MissingField("url: String.".to_string()));
    }

    Ok(ArgsPost {
        url: _url,
        request: _request,
    })
}

pub fn serialize_post_args(args: &ArgsPost) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: post Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_post_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_post_args<W: Write>(args: &ArgsPost, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("url", "String", "writing property");
    writer.write_string("url")?;
    writer.write_string(&args.url)?;
    writer.context().pop();
    writer.context().push("request", "Option<HttpRequest>", "writing property");
    writer.write_string("request")?;
    if args.request.is_some() {
        HttpRequest::write(args.request.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_post_result(result: &Option<HttpResponse>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: post Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_post_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_post_result<W: Write>(result: &Option<HttpResponse>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("post", "Option<HttpResponse>", "writing result");
    if result.is_some() {
        HttpResponse::write(result.as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn deserialize_post_result(result: &[u8]) -> Result<Option<HttpResponse>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: post Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("post", "Option<HttpResponse>", "reading function output");
    let mut object: Option<HttpResponse> = None;
    if !reader.is_next_nil()? {
        object = Some(HttpResponse::read(&mut reader)?);
    } else {
        object = None;
    }
    let res = object;
    reader.context().pop();
    Ok(res)
}
