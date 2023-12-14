use std::{
    ffi::{CStr, CString},
    sync::{atomic::AtomicI64, Arc},
    thread,
};

use crate::{
    ctp_command::CmdData, ctpimpl::check_make_dir, CommandResponse, CtpCommand, CtpError,
    CtpService, Kvpair, MdLogin, MdSubscribe,
};
use ctp_sys::{
    ascii_cstr_to_str_i8,
    md_api::{create_api, create_spi},
    print_rsp_info, set_cstr_from_str_truncate_i8, trading_day_from_ctp_trading_day,
    CThostFtdcMdApi, CThostFtdcReqUserLoginField, CtpAccountConfig,
};
use futures::StreamExt;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tracing::info;

pub struct CtpSys {
    account: Arc<CtpAccountConfig>,
    _request_id: AtomicI64,
    md_sender_revice: (
        UnboundedSender<CtpCommand>,
        Arc<Mutex<UnboundedReceiver<CtpCommand>>>,
    ),
}

impl CtpSys {
    pub fn new(
        account: CtpAccountConfig,
        md_sender_revice: (
            UnboundedSender<CtpCommand>,
            Arc<Mutex<UnboundedReceiver<CtpCommand>>>,
        ),
    ) -> Self {
        Self {
            account: Arc::new(account),
            md_sender_revice,
            _request_id: AtomicI64::new(0),
        }
    }
}

trait CtpCommandExecute {
    fn execute(&self, ctp_api: &mut CThostFtdcMdApi) -> CommandResponse;
}

impl CtpCommandExecute for MdLogin {
    fn execute(&self, ctp_api: &mut CThostFtdcMdApi) -> CommandResponse {
        let mut p_req_user_login_field = CThostFtdcReqUserLoginField::default();
        set_cstr_from_str_truncate_i8(&mut p_req_user_login_field.BrokerID, &self.broker_id);
        set_cstr_from_str_truncate_i8(&mut p_req_user_login_field.UserID, &self.user_id);
        set_cstr_from_str_truncate_i8(&mut p_req_user_login_field.Password, &self.password);
        ctp_api.req_user_login(&mut p_req_user_login_field, 1);
        String::from("login success").into()
    }
}

impl CtpCommandExecute for MdSubscribe {
    fn execute(&self, ctp_api: &mut CThostFtdcMdApi) -> CommandResponse {
        let mut v = vec![];
        for sybmol in &self.instrument_ids {
            let symbol = std::ffi::CString::new(sybmol.to_string()).unwrap();
            v.push(symbol);
        }
        let count = v.len() as i32;
        info!("MdSubscribe symbols={:?}", v);
        let ret = ctp_api.subscribe_market_data(v, count);

        format!("MdSubscribe success:{}", ret).into()
    }
}

/// 从 Request 中的到 response
pub fn cmd_dispatch(cmd: CtpCommand, ctp_api: &mut CThostFtdcMdApi) -> CommandResponse {
    match cmd.cmd_data {
        Some(CmdData::Mdlogin(param)) => param.execute(ctp_api),
        Some(CmdData::Mdsubscribe(param)) => param.execute(ctp_api),
        None => todo!(),
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

    fn add_subscribe(&self, symbols: Vec<String>) -> Result<Option<i64>, CtpError> {
        info!("subscribe:{:?}", symbols);
        // Implement the add_subscribe function here
        let _ = self
            .md_sender_revice
            .0
            .send(CtpCommand::new_md_subscribe(symbols));
        Ok(None)
    }

    fn init(&self) {
        info!("ctp sys init ...");

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

        // 注册前置机
        mdapi.register_front(CString::new(self.account.md_front.clone()).unwrap());
        info!("register front {}", self.account.md_front);
        // 初始化
        mdapi.init();
        info!("mdapi init succeed. ");

        let (sender, mut receiver) = mpsc::unbounded_channel::<CtpCommand>();

        // Command processing thread
        thread::spawn(move || {
            info!("in Command processing thread. ");
            while let Some(cmd) = receiver.blocking_recv() {
                let _response = cmd_dispatch(cmd, &mut mdapi);
            }
        });

        let accountc = self.account.clone();
        tokio::spawn(async move {
            let accountc = accountc.clone();

            // 处理登陆初始化查询
            while let Some(spi_msg) = stream.next().await {
                use ctp_sys::md_api::CThostFtdcMdSpiOutput::*;
                match spi_msg {
                    OnFrontConnected(_p) => {
                        info!("OnFrontConnected");
                        let cmd = CtpCommand::new_md_login(
                            &accountc.broker_id,
                            &accountc.account,
                            &accountc.password,
                            1,
                        );
                        let _ = sender.send(cmd);
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
                            info!("rec_c3 await");
                            let _ =
                                sender.send(CtpCommand::new_md_subscribe(vec!["ru2401".into()]));
                        } else {
                            info!("Trade RspUserLogin = {:?}", print_rsp_info!(&p.p_rsp_info));
                        }
                    }
                    OnRtnDepthMarketData(ref md) => {
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
        });

        // mdapi.release();
        // mdapi.join();
    }
}
