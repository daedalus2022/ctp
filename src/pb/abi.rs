/// 请求命令
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(oneof = "command_request::RequestData", tags = "1, 2")]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        /// 获取行情库版本
        #[prost(message, tag = "1")]
        Mdqversion(super::MdQversion),
        /// 订阅行情
        #[prost(message, tag = "2")]
        Mdsubscribe(super::MdSubscribe),
    }
}
/// ctp command
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CtpCommand {
    #[prost(oneof = "ctp_command::CmdData", tags = "1, 2")]
    pub cmd_data: ::core::option::Option<ctp_command::CmdData>,
}
/// Nested message and enum types in `CtpCommand`.
pub mod ctp_command {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CmdData {
        /// 订阅行情
        #[prost(message, tag = "1")]
        Mdsubscribe(super::MdSubscribe),
        /// 登录
        #[prost(message, tag = "2")]
        Mdlogin(super::MdLogin),
    }
}
/// 行情版本
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdQversion {}
/// 登录
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdLogin {
    /// 经纪公司代码
    #[prost(string, tag = "1")]
    pub broker_id: ::prost::alloc::string::String,
    /// 用户代码
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// 密码
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    /// 请求id
    #[prost(int64, tag = "4")]
    pub request_id: i64,
}
/// 订阅行情
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MdSubscribe {
    /// 合约代码池
    #[prost(string, repeated, tag = "1")]
    pub instrument_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 响应
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    /// 状态码
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// 消息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// 成功返回的values
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// 返回值
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag = "3")]
        Integer(i64),
        #[prost(double, tag = "4")]
        Float(f64),
        #[prost(bool, tag = "5")]
        Bool(bool),
    }
}
/// kv 值
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Kvpair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
