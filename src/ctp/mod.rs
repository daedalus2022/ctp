pub mod ctpsys;

use std::fs;

use tracing::info;

use crate::{error::CtpError, Kvpair};

fn check_make_dir(path: &String) {
    // 创建目录
    match fs::create_dir_all(path) {
        Ok(_) => info!("目录创建成功：{}", path),
        Err(e) => info!("无法创建目录：{}->{}", path, e),
    }
}

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

    ///
    fn init(&self);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ctp_sys::CtpAccountConfig;
    use tokio::sync::{mpsc, Mutex};
    use tracing::info;

    use super::ctpsys::CtpSys;
    use crate::CtpService;

    #[tokio::test]
    async fn ctp_sys_basic_interface_should_work() {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info")
        }
        // 初始化日志
        tracing_subscriber::fmt::init();

        let account = CtpAccountConfig {
            broker_id: "9999".to_string(),
            account: "-".to_string(),
            trade_front: "tcp://180.168.146.187:10201".to_string(),
            md_front: "tcp://180.168.146.187:10131".to_string(),
            // md_front: "tcp://180.168.146.187:10211".to_string(),
            name_server: "".to_string(),
            auth_code: "0000000000000000".to_string(),
            user_product_info: "".to_string(),
            app_id: "simnow_client_test".to_string(),
            password: "-".to_string(),
        };

        info!("完成保存查询结果");

        let (ctp_sender, ctp_sys_receiver) = mpsc::unbounded_channel::<String>();

        let ctp = CtpSys::new(
            account,
            (ctp_sender, Arc::new(Mutex::new(ctp_sys_receiver))),
        );

        // ctp.init();

        // let _ = ctp.add_subscribe(vec!["symbols".to_string()]);
        // tokio::time::sleep(std::time::Duration::from_secs(100)).await;

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
