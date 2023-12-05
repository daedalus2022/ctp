use std::ffi::CStr;

use ctp_sys::CThostFtdcMdApi;

use crate::{CtpError, CtpService, Kvpair};

#[derive(Default)]
pub struct CtpSys {
    mdapi: Option<Box<CThostFtdcMdApi>>,
}

impl CtpSys {
    pub fn new(mdapi: Box<CThostFtdcMdApi>) -> Self {
        Self { mdapi: Some(mdapi) }
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

    fn get_status(&self) -> Result<Option<Vec<Kvpair>>, CtpError> {
        // Implement the get_status function here
        todo!()
    }

    fn add_subscribe(&self, symbols: Vec<String>) -> Result<Option<Vec<Kvpair>>, CtpError> {
        // Implement the add_subscribe function here
        todo!()
    }
}
