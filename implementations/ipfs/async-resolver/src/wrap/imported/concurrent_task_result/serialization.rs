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
use crate::ConcurrentTaskResult;

use crate::{
    ConcurrentTaskStatus,
    get_concurrent_task_status_value,
    sanitize_concurrent_task_status_value
};

pub fn serialize_concurrent_task_result(args: &ConcurrentTaskResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ConcurrentTaskResult".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_concurrent_task_result(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_concurrent_task_result<W: Write>(args: &ConcurrentTaskResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("taskId", "i32", "writing property");
    writer.write_string("taskId")?;
    writer.write_i32(&args.task_id)?;
    writer.context().pop();
    writer.context().push("result", "Option<Vec<u8>>", "writing property");
    writer.write_string("result")?;
    writer.write_optional_bytes(&args.result)?;
    writer.context().pop();
    writer.context().push("error", "Option<String>", "writing property");
    writer.write_string("error")?;
    writer.write_optional_string(&args.error)?;
    writer.context().pop();
    writer.context().push("status", "ConcurrentTaskStatus", "writing property");
    writer.write_string("status")?;
    writer.write_i32(&(args.status as i32))?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_concurrent_task_result(args: &[u8]) -> Result<ConcurrentTaskResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ConcurrentTaskResult".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_concurrent_task_result(&mut reader)
}

pub fn read_concurrent_task_result<R: Read>(reader: &mut R) -> Result<ConcurrentTaskResult, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _task_id: i32 = 0;
    let mut _task_id_set = false;
    let mut _result: Option<Vec<u8>> = None;
    let mut _error: Option<String> = None;
    let mut _status: ConcurrentTaskStatus = ConcurrentTaskStatus::_MAX_;
    let mut _status_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "taskId" => {
                reader.context().push(&field, "i32", "type found, reading property");
                _task_id = reader.read_i32()?;
                _task_id_set = true;
                reader.context().pop();
            }
            "result" => {
                reader.context().push(&field, "Option<Vec<u8>>", "type found, reading property");
                _result = reader.read_optional_bytes()?;
                reader.context().pop();
            }
            "error" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _error = reader.read_optional_string()?;
                reader.context().pop();
            }
            "status" => {
                reader.context().push(&field, "ConcurrentTaskStatus", "type found, reading property");
                let mut value: ConcurrentTaskStatus = ConcurrentTaskStatus::_MAX_;
                if reader.is_next_string()? {
                    value = get_concurrent_task_status_value(&reader.read_string()?)?;
                } else {
                    value = ConcurrentTaskStatus::try_from(reader.read_i32()?)?;
                    sanitize_concurrent_task_status_value(value as i32)?;
                }
                _status = value;
                _status_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_task_id_set {
        return Err(DecodeError::MissingField("taskId: Int.".to_string()));
    }
    if !_status_set {
        return Err(DecodeError::MissingField("status: Concurrent_TaskStatus.".to_string()));
    }

    Ok(ConcurrentTaskResult {
        task_id: _task_id,
        result: _result,
        error: _error,
        status: _status,
    })
}
