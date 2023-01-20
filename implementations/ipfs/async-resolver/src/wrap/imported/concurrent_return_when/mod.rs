use polywrap_wasm_rs::EnumTypeError;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ConcurrentReturnWhen {
    FIRST_COMPLETED,
    ANY_COMPLETED,
    ALL_COMPLETED,
    _MAX_
}

pub fn sanitize_concurrent_return_when_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= ConcurrentReturnWhen::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'ConcurrentReturnWhen': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_concurrent_return_when_value(key: &str) -> Result<ConcurrentReturnWhen, EnumTypeError> {
    match key {
        "FIRST_COMPLETED" => Ok(ConcurrentReturnWhen::FIRST_COMPLETED),
        "ANY_COMPLETED" => Ok(ConcurrentReturnWhen::ANY_COMPLETED),
        "ALL_COMPLETED" => Ok(ConcurrentReturnWhen::ALL_COMPLETED),
        "_MAX_" => Ok(ConcurrentReturnWhen::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'ConcurrentReturnWhen': {}", err)))
    }
}

pub fn get_concurrent_return_when_key(value: ConcurrentReturnWhen) -> Result<String, EnumTypeError> {
    if sanitize_concurrent_return_when_value(value as i32).is_ok() {
        match value {
            ConcurrentReturnWhen::FIRST_COMPLETED => Ok("FIRST_COMPLETED".to_string()),
            ConcurrentReturnWhen::ANY_COMPLETED => Ok("ANY_COMPLETED".to_string()),
            ConcurrentReturnWhen::ALL_COMPLETED => Ok("ALL_COMPLETED".to_string()),
            ConcurrentReturnWhen::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'ConcurrentReturnWhen': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for ConcurrentReturnWhen {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<ConcurrentReturnWhen, Self::Error> {
        match v {
            x if x == ConcurrentReturnWhen::FIRST_COMPLETED as i32 => Ok(ConcurrentReturnWhen::FIRST_COMPLETED),
            x if x == ConcurrentReturnWhen::ANY_COMPLETED as i32 => Ok(ConcurrentReturnWhen::ANY_COMPLETED),
            x if x == ConcurrentReturnWhen::ALL_COMPLETED as i32 => Ok(ConcurrentReturnWhen::ALL_COMPLETED),
            x if x == ConcurrentReturnWhen::_MAX_ as i32 => Ok(ConcurrentReturnWhen::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'ConcurrentReturnWhen': {}", (v  as i32).to_string()))),
        }
    }
}
