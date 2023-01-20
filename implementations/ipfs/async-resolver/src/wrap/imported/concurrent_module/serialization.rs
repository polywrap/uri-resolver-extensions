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

use crate::{
    ConcurrentReturnWhen,
    get_concurrent_return_when_value,
    sanitize_concurrent_return_when_value
};
use crate::ConcurrentTaskResult;
use crate::{
    ConcurrentTaskStatus,
    get_concurrent_task_status_value,
    sanitize_concurrent_task_status_value
};
use crate::ConcurrentTask;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsResult {
    pub task_ids: Vec<i32>,
    pub return_when: ConcurrentReturnWhen,
}

pub fn deserialize_result_args(args: &[u8]) -> Result<ArgsResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: result Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _task_ids: Vec<i32> = vec![];
    let mut _task_ids_set = false;
    let mut _return_when: ConcurrentReturnWhen = ConcurrentReturnWhen::_MAX_;
    let mut _return_when_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "taskIds" => {
                reader.context().push(&field, "Vec<i32>", "type found, reading argument");
                _task_ids = reader.read_array(|reader| {
                    reader.read_i32()
                })?;
                _task_ids_set = true;
                reader.context().pop();
            }
            "returnWhen" => {
                reader.context().push(&field, "ConcurrentReturnWhen", "type found, reading argument");
                let mut value: ConcurrentReturnWhen = ConcurrentReturnWhen::_MAX_;
                if reader.is_next_string()? {
                    value = get_concurrent_return_when_value(&reader.read_string()?)?;
                } else {
                    value = ConcurrentReturnWhen::try_from(reader.read_i32()?)?;
                    sanitize_concurrent_return_when_value(value as i32)?;
                }
                _return_when = value;
                _return_when_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_task_ids_set {
        return Err(DecodeError::MissingField("taskIds: [Int].".to_string()));
    }
    if !_return_when_set {
        return Err(DecodeError::MissingField("returnWhen: Concurrent_ReturnWhen.".to_string()));
    }

    Ok(ArgsResult {
        task_ids: _task_ids,
        return_when: _return_when,
    })
}

pub fn serialize_result_args(args: &ArgsResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: result Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_result_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_result_args<W: Write>(args: &ArgsResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("taskIds", "Vec<i32>", "writing property");
    writer.write_string("taskIds")?;
    writer.write_array(&args.task_ids, |writer, item| {
        writer.write_i32(item)
    })?;
    writer.context().pop();
    writer.context().push("returnWhen", "ConcurrentReturnWhen", "writing property");
    writer.write_string("returnWhen")?;
    writer.write_i32(&(args.return_when as i32))?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_result_result(result: &Vec<ConcurrentTaskResult>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: result Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_result_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_result_result<W: Write>(result: &Vec<ConcurrentTaskResult>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("result", "Vec<ConcurrentTaskResult>", "writing result");
    writer.write_array(&result, |writer, item| {
        ConcurrentTaskResult::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_result_result(result: &[u8]) -> Result<Vec<ConcurrentTaskResult>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: result Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("result", "Vec<ConcurrentTaskResult>", "reading function output");
    let res = reader.read_array(|reader| {
        let object = ConcurrentTaskResult::read(reader)?;
        Ok(object)
    })?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsStatus {
    pub task_ids: Vec<i32>,
}

pub fn deserialize_status_args(args: &[u8]) -> Result<ArgsStatus, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: status Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _task_ids: Vec<i32> = vec![];
    let mut _task_ids_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "taskIds" => {
                reader.context().push(&field, "Vec<i32>", "type found, reading argument");
                _task_ids = reader.read_array(|reader| {
                    reader.read_i32()
                })?;
                _task_ids_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_task_ids_set {
        return Err(DecodeError::MissingField("taskIds: [Int].".to_string()));
    }

    Ok(ArgsStatus {
        task_ids: _task_ids,
    })
}

pub fn serialize_status_args(args: &ArgsStatus) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: status Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_status_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_status_args<W: Write>(args: &ArgsStatus, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("taskIds", "Vec<i32>", "writing property");
    writer.write_string("taskIds")?;
    writer.write_array(&args.task_ids, |writer, item| {
        writer.write_i32(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_status_result(result: &Vec<ConcurrentTaskStatus>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: status Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_status_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_status_result<W: Write>(result: &Vec<ConcurrentTaskStatus>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("status", "Vec<ConcurrentTaskStatus>", "writing result");
    writer.write_array(&result, |writer, item| {
        writer.write_i32(&(*item as i32))
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_status_result(result: &[u8]) -> Result<Vec<ConcurrentTaskStatus>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: status Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("status", "Vec<ConcurrentTaskStatus>", "reading function output");
    let res = reader.read_array(|reader| {
        let mut value: ConcurrentTaskStatus = ConcurrentTaskStatus::_MAX_;
        if reader.is_next_string()? {
            value = get_concurrent_task_status_value(&reader.read_string()?)?;
        } else {
            value = ConcurrentTaskStatus::try_from(reader.read_i32()?)?;
            sanitize_concurrent_task_status_value(value as i32)?;
        }
        Ok(value)
    })?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSchedule {
    pub tasks: Vec<ConcurrentTask>,
}

pub fn deserialize_schedule_args(args: &[u8]) -> Result<ArgsSchedule, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: schedule Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _tasks: Vec<ConcurrentTask> = vec![];
    let mut _tasks_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "tasks" => {
                reader.context().push(&field, "Vec<ConcurrentTask>", "type found, reading argument");
                _tasks = reader.read_array(|reader| {
                    let object = ConcurrentTask::read(reader)?;
                    Ok(object)
                })?;
                _tasks_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_tasks_set {
        return Err(DecodeError::MissingField("tasks: [Concurrent_Task].".to_string()));
    }

    Ok(ArgsSchedule {
        tasks: _tasks,
    })
}

pub fn serialize_schedule_args(args: &ArgsSchedule) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: schedule Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_schedule_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_schedule_args<W: Write>(args: &ArgsSchedule, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("tasks", "Vec<ConcurrentTask>", "writing property");
    writer.write_string("tasks")?;
    writer.write_array(&args.tasks, |writer, item| {
        ConcurrentTask::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_schedule_result(result: &Vec<i32>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: schedule Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_schedule_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_schedule_result<W: Write>(result: &Vec<i32>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("schedule", "Vec<i32>", "writing result");
    writer.write_array(&result, |writer, item| {
        writer.write_i32(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_schedule_result(result: &[u8]) -> Result<Vec<i32>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: schedule Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("schedule", "Vec<i32>", "reading function output");
    let res = reader.read_array(|reader| {
        reader.read_i32()
    })?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsAbort {
    pub task_ids: Vec<String>,
}

pub fn deserialize_abort_args(args: &[u8]) -> Result<ArgsAbort, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: abort Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _task_ids: Vec<String> = vec![];
    let mut _task_ids_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "taskIds" => {
                reader.context().push(&field, "Vec<String>", "type found, reading argument");
                _task_ids = reader.read_array(|reader| {
                    reader.read_string()
                })?;
                _task_ids_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_task_ids_set {
        return Err(DecodeError::MissingField("taskIds: [String].".to_string()));
    }

    Ok(ArgsAbort {
        task_ids: _task_ids,
    })
}

pub fn serialize_abort_args(args: &ArgsAbort) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: abort Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_abort_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_abort_args<W: Write>(args: &ArgsAbort, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("taskIds", "Vec<String>", "writing property");
    writer.write_string("taskIds")?;
    writer.write_array(&args.task_ids, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_abort_result(result: &Vec<bool>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: abort Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_abort_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_abort_result<W: Write>(result: &Vec<bool>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("abort", "Vec<bool>", "writing result");
    writer.write_array(&result, |writer, item| {
        writer.write_bool(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_abort_result(result: &[u8]) -> Result<Vec<bool>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: abort Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("abort", "Vec<bool>", "reading function output");
    let res = reader.read_array(|reader| {
        reader.read_bool()
    })?;
    reader.context().pop();
    Ok(res)
}
