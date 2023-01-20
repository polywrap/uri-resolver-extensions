use polywrap_wasm_rs::EnumTypeError;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ConcurrentTaskStatus {
    PENDING,
    RUNNING,
    COMPLETED,
    CANCELLED,
    FAILED,
    _MAX_
}

pub fn sanitize_concurrent_task_status_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= ConcurrentTaskStatus::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'ConcurrentTaskStatus': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_concurrent_task_status_value(key: &str) -> Result<ConcurrentTaskStatus, EnumTypeError> {
    match key {
        "PENDING" => Ok(ConcurrentTaskStatus::PENDING),
        "RUNNING" => Ok(ConcurrentTaskStatus::RUNNING),
        "COMPLETED" => Ok(ConcurrentTaskStatus::COMPLETED),
        "CANCELLED" => Ok(ConcurrentTaskStatus::CANCELLED),
        "FAILED" => Ok(ConcurrentTaskStatus::FAILED),
        "_MAX_" => Ok(ConcurrentTaskStatus::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'ConcurrentTaskStatus': {}", err)))
    }
}

pub fn get_concurrent_task_status_key(value: ConcurrentTaskStatus) -> Result<String, EnumTypeError> {
    if sanitize_concurrent_task_status_value(value as i32).is_ok() {
        match value {
            ConcurrentTaskStatus::PENDING => Ok("PENDING".to_string()),
            ConcurrentTaskStatus::RUNNING => Ok("RUNNING".to_string()),
            ConcurrentTaskStatus::COMPLETED => Ok("COMPLETED".to_string()),
            ConcurrentTaskStatus::CANCELLED => Ok("CANCELLED".to_string()),
            ConcurrentTaskStatus::FAILED => Ok("FAILED".to_string()),
            ConcurrentTaskStatus::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'ConcurrentTaskStatus': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for ConcurrentTaskStatus {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<ConcurrentTaskStatus, Self::Error> {
        match v {
            x if x == ConcurrentTaskStatus::PENDING as i32 => Ok(ConcurrentTaskStatus::PENDING),
            x if x == ConcurrentTaskStatus::RUNNING as i32 => Ok(ConcurrentTaskStatus::RUNNING),
            x if x == ConcurrentTaskStatus::COMPLETED as i32 => Ok(ConcurrentTaskStatus::COMPLETED),
            x if x == ConcurrentTaskStatus::CANCELLED as i32 => Ok(ConcurrentTaskStatus::CANCELLED),
            x if x == ConcurrentTaskStatus::FAILED as i32 => Ok(ConcurrentTaskStatus::FAILED),
            x if x == ConcurrentTaskStatus::_MAX_ as i32 => Ok(ConcurrentTaskStatus::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'ConcurrentTaskStatus': {}", (v  as i32).to_string()))),
        }
    }
}
