mod abi;

pub use abi::*;
use http::StatusCode;

use crate::CtpError;

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(value::Value::String(value)),
        }
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(value)),
        }
    }
}

impl From<String> for CommandResponse {
    fn from(value: String) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![value.into()],
            ..Default::default()
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![value],
            ..Default::default()
        }
    }
}

impl From<CtpError> for CommandResponse {
    fn from(error: CtpError) -> Self {
        match error {
            CtpError::CtpServiceError(e) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
                message: e,
                ..Default::default()
            },
            CtpError::CtpGetVersionError(e) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
                message: e,
                ..Default::default()
            },
        }
    }
}
