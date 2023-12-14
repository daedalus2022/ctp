use thiserror::Error;

/// ctp 错误定义
#[derive(Error, Debug, PartialEq)]
pub enum CtpError {
    #[error("CTP service error: `{0}`")]
    CtpServiceError(String),

    #[error("CTP service get version error: `{0}`")]
    CtpGetVersionError(String),
}
