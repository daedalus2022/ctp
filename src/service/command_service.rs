///
/// 对 Command 的处理的抽象
///
pub trait CommandService {
    /// 处理 Command, 返回 Response
    fn execute(self, ctp: &impl CtpService) -> CommandResponse;
}

///
/// CTP 服务
///
pub trait CtpService {}
