use crate::{command_request::RequestData, CommandRequest, CommandResponse, CtpService};

mod command_service;
pub use command_service::*;

/// 从 Request 中的到 response
pub fn dispatch(cmd: CommandRequest, ctp: &impl CtpService) -> CommandResponse {
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
