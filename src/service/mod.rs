use std::sync::Arc;

use crate::{
    command_request::RequestData, ctpimpl::CtpSys, CommandRequest, CommandResponse, CtpService,
};

mod command_service;
pub use command_service::*;

/// Service 结构
/// ctp 服务
/// cpt 结果保存位置，以供查询
pub struct Service<CtpService = CtpSys> {
    inner: Arc<ServiceInner<CtpService>>,
}

/// 实现 clone
impl<CtpService> Clone for Service<CtpService> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct ServiceInner<CtpService> {
    ctp: CtpService,
}

impl<Ctp: CtpService> Service<Ctp> {
    pub fn new(ctp: Ctp) -> Self {
        Self {
            inner: Arc::new(ServiceInner { ctp }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        dispatch(cmd, &self.inner.ctp)
    }
}

/// 从 Request 中的到 response
pub fn dispatch(cmd: CommandRequest, ctp: &impl CtpService) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Mdqversion(param)) => param.execute(ctp),
        Some(RequestData::Mdsubscribe(param)) => param.execute(ctp),
        None => todo!(),
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use ctp_sys::CtpAccountConfig;
    use tokio::sync::{mpsc, Mutex};

    use crate::{CommandRequest, CtpCommand, CtpSys};

    use super::Service;

    #[test]
    fn service_should_works() {
        let account = CtpAccountConfig {
            broker_id: "9999".to_string(),
            account: "15801632955".to_string(),
            trade_front: "tcp://180.168.146.187:10201".to_string(),
            md_front: "tcp://180.168.146.187:10131".to_string(),
            // md_front: "tcp://180.168.146.187:10211".to_string(),
            name_server: "".to_string(),
            auth_code: "0000000000000000".to_string(),
            user_product_info: "".to_string(),
            app_id: "simnow_client_test".to_string(),
            password: "87406037".to_string(),
        };

        let (ctp_sender, ctp_sys_receiver) = mpsc::unbounded_channel::<CtpCommand>();

        let ctp = CtpSys::new(
            account,
            (ctp_sender, Arc::new(Mutex::new(ctp_sys_receiver))),
        );

        let service = Service::new(ctp);

        let cmd = CommandRequest::new_md_q_version();

        let response = service.execute(cmd);

        println!("version:{:?}", response);

        let cmd = CommandRequest::new_md_subscribe(vec!["ru2312".into()]);

        let response = service.execute(cmd);
        println!("subscribe:{:?}", response);
    }
}
