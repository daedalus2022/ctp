mod ctpsys;

use std::ffi::CStr;

use crate::{error::CtpError, CommandResponse};

///
/// CTP 服务
///
pub trait CtpService {
    /// 获取 ctp 版本信息
    fn get_version(&self) -> Result<Option<String>, CtpError>;
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use super::ctpsys::CtpSys;
    use crate::CtpService;

    #[test]
    fn ctp_sys_basic_interface_should_work() {
        let ctp = CtpSys::new();

        test_basic_interface(ctp);
    }

    fn test_basic_interface(ctp: impl CtpService) {
        // 版本接口
        assert!(ctp.get_version().is_ok());
        if let Ok(Some(version)) = ctp.get_version() {
            info!("ctp version is: {}", version)
        }
    }
}
