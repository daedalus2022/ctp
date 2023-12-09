use std::sync::Arc;

use crate::{
    command_request::RequestData, ctp::ctpsys::CtpSys, memory::MemTable, CommandRequest,
    CommandResponse, CtpService,
};

mod command_service;
pub use command_service::*;

/// Service 结构
/// ctp 服务
/// cpt 结果保存位置，以供查询
pub struct _Service<CtpService = CtpSys, Store = MemTable> {
    inner: Arc<_ServiceInner<CtpService, Store>>,
}

pub struct _ServiceInner<CtpService, Store> {
    ctp: CtpService,
    store: Store,
}

/// 从 Request 中的到 response
pub fn _dispatch(cmd: CommandRequest, ctp: &impl CtpService) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Mdqversion(param)) => param.execute(ctp),
        None => todo!(),
    }
}

#[cfg(test)]
mod test {
    // use crate::CtpSys;

    // /// .
    // #[test]
    // fn service_should_works() {
    //     let ctp_sys = CtpSys::default();

    //     let service = Service::new(ctp_sys);
    // }
}
