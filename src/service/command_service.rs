use crate::{
    command_request::RequestData, CommandRequest, CommandResponse, CtpService, MdQversion,
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
}

/// 版本
impl CommandService for MdQversion {
    fn execute(self, ctp: &impl CtpService) -> CommandResponse {
        match ctp.get_version() {
            Ok(Some(_value)) => todo!(),
            Ok(None) => todo!(),
            Err(_) => todo!(),
        }
    }
}
