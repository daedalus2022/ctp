use std::ffi::CStr;

use ctp_sys::CThostFtdcMdApi;

use crate::{CtpError, CtpService};

#[derive(Default)]
pub struct CtpSys {}

impl CtpSys {
    pub fn new() -> Self {
        Self {}
    }
}

impl CtpService for CtpSys {
    /// 获取 ctp 版本信息
    fn get_version(&self) -> Result<Option<String>, CtpError> {
        unsafe {
            if let Ok(version) = CStr::from_ptr(CThostFtdcMdApi::GetApiVersion()).to_str() {
                return Ok(Some(version.to_string()));
            }
        }

        Err(CtpError::CtpServiceError("get version error".into()))
    }
}
