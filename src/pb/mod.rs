mod abi;

pub use abi::*;
use http::StatusCode;

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(value::Value::String(value)),
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
