// use std::{ffi::{CString, c_int, c_void}, fmt};
// use ctp_sys::{CThostFtdcMdApi, CThostFtdcMdSpi, CThostFtdcMdApi_Init, CThostFtdcRspUserLoginField, TThostFtdcRequestIDType, CThostFtdcUserLogoutField, CThostFtdcSpecificInstrumentField, CThostFtdcDepthMarketDataField, CThostFtdcForQuoteRspField, CThostFtdcMdApi_RegisterSpi};

// use crate::{RspResult, from_rsp_result_to_string};
// ///
// /// 行情API
// ///
// pub trait MdService {
//     ///
//     /// 创建
//     ///
//     fn new(flow_path: String, use_udp: bool, use_multicast: bool) -> Self;
//     ///
//     /// 初始化
//     /// @remark 初始化运行环境,只有调用后,接口才开始工作
//     ///
//     fn init(&mut self);
//     ///
//     /// 注册订阅服务
//     ///
//     fn register_spi(&mut self, md_spi: Box<dyn MdSpi>);
//     // fn join(&mut self) -> ApiResult;
//     // fn get_trading_day<'a>(&mut self) -> &'a CStr;
//     // fn register_front(&mut self, front_socket_address: CString);
//     // fn register_name_server(&mut self, name_server: CString);
//     // fn register_fens_user_info(&mut self, fens_user_info: &CThostFtdcFensUserInfoField);

//     // fn subscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult;
//     // fn unsubscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult;
//     // fn subscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult;
//     // fn unsubscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult;
//     // fn req_user_login(&mut self, req_user_login: &CThostFtdcReqUserLoginField, request_id: TThostFtdcRequestIDType) -> ApiResult;
//     // fn req_user_logout(&mut self, req_user_logout: &CThostFtdcUserLogoutField, request_id: TThostFtdcRequestIDType) -> ApiResult;
// }

// ///
// /// 行情
// ///
// pub struct MdApi{
//     pub md_api_ptr: *mut CThostFtdcMdApi,
//     pub registered_spi: Option<*mut CThostFtdcMdSpi>,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum DisconnectionReason {
//     ReadError = 0x1001,
//     WriteError = 0x1002,
//     HeartbeatTimeout = 0x2001,
//     HeartbeatSendError = 0x2002,
//     ErrorMessageReceived = 0x2003,
//     Unknown = 0x0000,
// }

// impl std::convert::From<c_int> for DisconnectionReason {
//     fn from(reason: c_int) -> DisconnectionReason {
//         match reason {
//             0x1001 => DisconnectionReason::ReadError,
//             0x1002 => DisconnectionReason::WriteError,
//             0x2001 => DisconnectionReason::HeartbeatTimeout,
//             0x2002 => DisconnectionReason::HeartbeatSendError,
//             0x2003 => DisconnectionReason::ErrorMessageReceived,
//             _ => DisconnectionReason::Unknown,
//         }
//     }
// }

// impl fmt::Display for DisconnectionReason {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use DisconnectionReason::*;
//         match *self {
//             ReadError => f.write_str("read error"),
//             WriteError => f.write_str("write error"),
//             HeartbeatTimeout => f.write_str("heartbeat timeout"),
//             HeartbeatSendError => f.write_str("heatbeat send error"),
//             ErrorMessageReceived => f.write_str("error message received"),
//             Unknown => f.write_str("unknown"),
//         }
//     }
// }
// pub trait MdSpi : Send {
//     fn on_front_connected(&mut self) {
//         println!("on_front_connected");
//     }

//     fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
//         println!("on_front_disconnected: {:?}", reason);
//     }

//     #[allow(unused_variables)]
//     fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
//         println!("on_rsp_user_login: {:?}, {}, {:?}, {:?}", rsp_user_login, from_rsp_result_to_string(&result), request_id, is_last);
//     }

//     #[allow(unused_variables)]
//     fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
//         println!("on_rsp_user_logout: {:?}, {}, {:?}, {:?}", rsp_user_logout, from_rsp_result_to_string(&result), request_id, is_last);
//     }

//     #[allow(unused_variables)]
//     fn on_rsp_error(&mut self, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
//         println!("on_rsp_error: {}, {:?}, {:?}", from_rsp_result_to_string(&result), request_id, is_last);
//     }

//     #[allow(unused_variables)]
//     fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
//         println!("on_rsp_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
//     }

//     #[allow(unused_variables)]
//     fn on_rsp_un_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
//         println!("on_rsp_un_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
//     }

//     #[allow(unused_variables)]
//     fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
//         println!("on_rsp_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
//     }

//     #[allow(unused_variables)]
//     fn on_rsp_un_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
//         println!("on_rsp_un_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
//     }

//     #[allow(unused_variables)]
//     fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&CThostFtdcDepthMarketDataField>) {
//         println!("on_rtn_depth_market_data: {:?}", depth_market_data);
//     }

//     #[allow(unused_variables)]
//     fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&CThostFtdcForQuoteRspField>) {
//         println!("on_rtn_for_quote_rsp: {:?}", for_quote_rsp);
//     }
// }

// impl MdService for MdApi{
//     ///
//     /// 创建
//     ///
//     fn new(flow_path: String, use_udp: bool, use_multicast: bool) -> Self {
//         let flow_path_cstring= CString::new(flow_path).unwrap();
//         let flow_path_ptr = flow_path_cstring.into_raw();
//         let api = unsafe { CThostFtdcMdApi::CreateFtdcMdApi(flow_path_ptr, use_udp, use_multicast) };
//         let flow_path = unsafe { CString::from_raw(flow_path_ptr) };
//         drop(flow_path);
//         MdApi{ md_api_ptr: api, registered_spi: None }
//     }

//     ///
//     /// 初始化
//     ///
//     fn init(&mut self) {
//         unsafe { CThostFtdcMdApi_Init(self.md_api_ptr) };
//     }

//     fn register_spi(&mut self, md_spi: Box<dyn MdSpi>) {
//         let last_registered_spi_ptr: Option<*mut CThostFtdcMdSpi> = self.registered_spi.take();
//         let md_spi_ptr = Box::into_raw(md_spi);
//         let spi_ptr = Box::into_raw(Box::new(CThostFtdcMdSpi{ _address: todo!() }));
//         unsafe { CThostFtdcMdApi_RegisterSpi(self.md_api_ptr, spi_ptr) };
//         self.registered_spi = Some(spi_ptr);
//         if let Some(last_registered_spi_ptr) = last_registered_spi_ptr {
//             unsafe {
//                 let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
//                 drop(last_registered_spi);
//             }
//         };
//     }
// }

// pub struct SpiApi{

// }

// impl MdSpi for SpiApi{

// }

// // impl CThostFtdcMdSpi {
// //     #[inline]
// //     pub unsafe fn OnFrontConnected(&mut self) {
// //         CThostFtdcMdSpi_OnFrontConnected(self)
// //     }
// //     #[inline]
// //     pub unsafe fn new() -> Self {
// //         let mut __bindgen_tmp: std::mem::MaybeUninit<CThostFtdcMdSpi> = ::std::mem::MaybeUninit::uninit();
// //         CThostFtdcMdSpi_CThostFtdcMdSpi(__bindgen_tmp.as_mut_ptr());
// //         __bindgen_tmp.assume_init()
// //     }
// // }

// // #[cfg(test)]
// // mod test{
// //     use ctp_sys::CThostFtdcMdApi;

// //     use crate::{MdApi, MdService, SpiApi};

// //     #[test]
// //     pub fn init_works(){

// //         let md_api = MdApi::new("".into(), false, false);
// //         md_api.register_spi(Box::new(SpiApi));
// //         md_api.register_front(std::ffi::CString::new("tcp://180.168.146.187:10131").unwrap());
// //         md_api.init();

// //         // let md_api = Box::from_raw(md.md_api_ptr);
// //         // CThostFtdcMdApi::GetApiVersion()
// //     }
// // }
