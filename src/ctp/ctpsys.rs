use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use crate::{ctp::check_make_dir, CtpError, CtpService, Kvpair};
use ctp_sys::{
    ascii_cstr_to_str_i8,
    md_api::{create_api, create_spi},
    print_rsp_info, set_cstr_from_str_truncate_i8, trading_day_from_ctp_trading_day,
    CThostFtdcMdApi, CThostFtdcReqUserLoginField, CtpAccountConfig,
};
use futures::StreamExt;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tracing::info;

pub struct CtpSys {
    account: Arc<CtpAccountConfig>,
    md_sender_revice: (
        UnboundedSender<String>,
        Arc<Mutex<UnboundedReceiver<String>>>,
    ),
}

impl CtpSys {
    pub fn new(
        account: CtpAccountConfig,
        md_sender_revice: (
            UnboundedSender<String>,
            Arc<Mutex<UnboundedReceiver<String>>>,
        ),
    ) -> Self {
        Self {
            account: Arc::new(account),
            md_sender_revice,
        }
    }
}

impl CtpService for CtpSys {
    /// 获取 ctp 版本信息
    fn get_version(&self) -> Result<Option<String>, CtpError> {
        unsafe {
            if let Ok(version) = CStr::from_ptr(CThostFtdcMdApi::GetApiVersion()).to_str() {
                return Ok(Some(version.to_string()));
            }
        }

        Err(CtpError::CtpServiceError("get version error".into()))
    }

    fn get_status(&self) -> Result<Option<Vec<Kvpair>>, CtpError> {
        // Implement the get_status function here
        todo!()
    }

    fn add_subscribe(&self, _symbols: Vec<String>) -> Result<Option<Vec<Kvpair>>, CtpError> {
        // Implement the add_subscribe function here
        let _ = self.md_sender_revice.0.send("abc".to_string());
        Ok(None)
    }

    fn init(&self) {
        info!("ctp sys init ...");
        let request_id: i32 = 0;
        // let mut get_request_id = || {
        //     request_id += 1;
        //     request_id
        // };

        // 本地缓存
        let flow_path = format!(
            ".cache/ctp_futures_md_flow_{}_{}/",
            self.account.broker_id.clone(),
            self.account.account
        );
        check_make_dir(&flow_path);

        // 创建行情服务
        let mut mdapi = create_api(&flow_path, false, false);

        // 行情接收
        let (spi_stream, spi) = create_spi();
        let mut stream = {
            mdapi.register_spi(spi);
            spi_stream
        };

        let acount_c1 = self.account.clone();

        let rec_c1 = self.md_sender_revice.1.clone();
        tokio::spawn(async move {
            let acount_c2 = acount_c1.clone();
            let rec_c2 = rec_c1.clone();

            // 注册前置机
            mdapi.register_front(CString::new(acount_c2.md_front.clone()).unwrap());
            info!("register front {}", acount_c2.md_front);
            // 初始化
            mdapi.init();
            info!("mdapi init succeed. ");
            // 处理登陆初始化查询
            while let Some(spi_msg) = stream.next().await {
                let ac1 = acount_c1.clone();
                let rec_c2_clone = rec_c2.clone();
                let mut rec_c3 = rec_c2_clone.lock().await;
                use ctp_sys::md_api::CThostFtdcMdSpiOutput::*;
                match spi_msg {
                    OnFrontConnected(_p) => {
                        let mut req = CThostFtdcReqUserLoginField::default();
                        set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ac1.broker_id);
                        set_cstr_from_str_truncate_i8(&mut req.UserID, &ac1.account);
                        set_cstr_from_str_truncate_i8(&mut req.Password, &ac1.password);
                        mdapi.req_user_login(&mut req, request_id);
                        info!("OnFrontConnected");
                    }
                    OnFrontDisconnected(p) => {
                        info!("on front disconnected {:?} 直接Exit ", p);
                        std::process::exit(-1);
                    }
                    OnRspUserLogin(ref p) => {
                        info!("OnRspUserLogin");
                        if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                            let _u = p.p_rsp_user_login.unwrap();

                            let _d = chrono::Local::now();

                            while let Some(_x) = rec_c3.recv().await {
                                let mut v = vec![];
                                let mut d = chrono::Local::now();
                                for _i in 0..1 {
                                    d = d.checked_add_months(chrono::Months::new(1)).unwrap();
                                    let symbol = std::ffi::CString::new(format!(
                                        "ru{}",
                                        d.format("%Y%m").to_string().trim_start_matches("20")
                                    ))
                                    .unwrap();
                                    v.push(symbol);
                                }
                                let count = v.len() as i32;
                                info!("symbols={:?}", v);
                                let ret = mdapi.subscribe_market_data(v, count);
                                info!("Subscribe ret = {}", ret);
                            }

                            // for x in self.md_sender_revice.1.iter() {
                            //     let v = vec![];
                            //     for _i in 0..1 {
                            //         d = d.checked_add_months(chrono::Months::new(1)).unwrap();
                            //         let symbol = std::ffi::CString::new(format!(
                            //             "ru{}",
                            //             d.format("%Y%m").to_string().trim_start_matches("20")
                            //         ))
                            //         .unwrap();
                            //         v.push(symbol);
                            //     }
                            //     let count = v.len() as i32;
                            //     info!("symbols={:?}", v);
                            //     let ret = mdapi.subscribe_market_data(v, count);
                            //     {
                            //         let ret = mdapi.subscribe_market_data(
                            //             vec![
                            //                 std::ffi::CString::new(String::from("ru2402")).unwrap()
                            //             ],
                            //             1,
                            //         );
                            //     }
                            //     info!("Subscribe ret = {}", ret);
                            // }
                        } else {
                            info!("Trade RspUserLogin = {:?}", print_rsp_info!(&p.p_rsp_info));
                        }
                    }
                    OnRtnDepthMarketData(ref md) => {
                        {
                            let ret = mdapi.subscribe_market_data(
                                vec![std::ffi::CString::new(String::from("ru2402")).unwrap()],
                                1,
                            );
                            info!("Subscribe ret = {}", ret);
                        }

                        info!("md={:?}", md);

                        if let Some(dmd) = md.p_depth_market_data {
                            info!(
                                "交易日期: TradingDay:{:?}",
                                trading_day_from_ctp_trading_day(&dmd.TradingDay)
                            );
                            info!(
                                "品种（保留字段）: reserve1:{:?}",
                                ascii_cstr_to_str_i8(&dmd.reserve1)
                            );
                            info!(
                                "交易所代码: ExchangeID::{:?}",
                                ascii_cstr_to_str_i8(&dmd.ExchangeID)
                            );
                            info!(
                                "空（保留字段）: reserve2::{:?}",
                                ascii_cstr_to_str_i8(&dmd.reserve2)
                            );
                            info!("最新价: LastPrice::{:?}", &dmd.LastPrice);
                            info!(
                                "上次结算价: PreSettlementPrice:::{:?}",
                                &dmd.PreSettlementPrice
                            );
                            info!("昨收盘: PreClosePrice:::{:?}", &dmd.PreClosePrice);
                            info!("昨持仓量: PreOpenInterest:::{:?}", &dmd.PreOpenInterest);
                            info!("今开盘: OpenPrice:::{:?}", &dmd.OpenPrice);
                            info!("最高价: HighestPrice:::{:?}", &dmd.HighestPrice);
                            info!("最低价: LowestPrice:::{:?}", &dmd.LowestPrice);
                            info!("数量: Volume:::{:?}", &dmd.Volume);
                            info!("成交金额: Turnover:::{:?}", &dmd.Turnover);
                            info!("持仓量: OpenInterest:::{:?}", &dmd.OpenInterest);
                            info!("今收盘: ClosePrice:::{:?}", &dmd.ClosePrice);
                            info!("本次结算价: SettlementPrice::::{:?}", &dmd.SettlementPrice);
                            info!("涨停板价: UpperLimitPrice:::{:?}", &dmd.UpperLimitPrice);
                            info!("跌停板价: LowerLimitPrice:::{:?}", &dmd.LowerLimitPrice);
                            info!("昨虚实度: PreDelta:::{:?}", &dmd.PreDelta);
                            info!("今虚实度: CurrDelta:::{:?}", &dmd.CurrDelta);
                            info!(
                                "最后修改时间: UpdateTime:::{:?}",
                                ascii_cstr_to_str_i8(&dmd.UpdateTime)
                            );
                            info!("最后修改毫秒: UpdateMillisec:::{:?}", &dmd.UpdateMillisec);
                            info!("当日均价: AveragePrice:::{:?}", &dmd.AveragePrice);
                            info!(
                                "业务日期: ActionDay:::{:?}",
                                trading_day_from_ctp_trading_day(&dmd.ActionDay)
                            );
                            info!(
                                "合约代码: InstrumentID:::{:?}",
                                ascii_cstr_to_str_i8(&dmd.InstrumentID)
                            );
                            info!(
                                "合约在交易所的代码: ExchangeInstID:::{:?}",
                                ascii_cstr_to_str_i8(&dmd.ExchangeInstID)
                            );
                            info!("上带价: BandingUpperPrice:::{:?}", &dmd.BandingUpperPrice);
                            info!("下带价: BandingLowerPrice:::{:?}", &dmd.BandingLowerPrice);
                        }
                    }
                    _ => {}
                }
            }
            mdapi.release();
            mdapi.join();
        });
    }
}
