mod ctpsys;

use crate::{error::CtpError, Kvpair};

///
/// CTP 服务
///
pub trait CtpService {
    /// 获取 ctp 版本信息
    fn get_version(&self) -> Result<Option<String>, CtpError>;

    /// 获取服务器状态
    fn get_status(&self) -> Result<Option<Vec<Kvpair>>, CtpError>;

    /// 添加订阅 [IF2312], 返回 topic ID，通过消费 topicId处理数据
    fn add_subscribe(&self, symbols: Vec<String>) -> Result<Option<Vec<Kvpair>>, CtpError>;
}

#[cfg(test)]
mod tests {
    use ctp_sys::md_api::{create_api, create_spi};
    use tracing::info;

    use super::ctpsys::CtpSys;
    use crate::CtpService;

    #[test]
    fn ctp_sys_basic_interface_should_work() {
        let mdapi = create_api(".", false, false);

        let p_spi = create_spi();

        mdapi.register_spi(p_spi);

        let ctp = CtpSys::new(mdapi);

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
