use crate::{
    command_request::RequestData, CommandRequest, CommandResponse, CtpError, CtpService,
    MdQversion, MdSubscribe, Value,
};

///
/// 对 Command 的处理的抽象
///
pub trait CommandService {
    /// 处理 Command, 返回 Response
    fn execute(self, ctp: &impl CtpService) -> CommandResponse;
}

impl CommandRequest {
    /// 获取版本命令
    pub fn new_md_q_version() -> Self {
        Self {
            request_data: Some(RequestData::Mdqversion(MdQversion {})),
        }
    }

    // 行情订阅
    pub fn new_md_subscribe(instrument_ids: Vec<String>) -> Self {
        Self {
            request_data: Some(RequestData::Mdsubscribe(MdSubscribe { instrument_ids })),
        }
    }
}

/// 版本
impl CommandService for MdQversion {
    fn execute(self, ctp: &impl CtpService) -> CommandResponse {
        match ctp.get_version() {
            Ok(Some(value)) => value.into(),
            Ok(None) => CtpError::CtpGetVersionError("get version is none".into()).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for MdSubscribe {
    fn execute(self, ctp: &impl CtpService) -> CommandResponse {
        match ctp.add_subscribe(self.instrument_ids) {
            Ok(Some(r)) => Value::from(r).into(),
            Ok(None) => CtpError::CtpGetVersionError("get version is none".into()).into(),
            Err(e) => e.into(),
        }
    }
}
