use crate::common::error::ServerError;
use crate::common::error::ServerError::BusinessError;
use crate::common::log_info::{process_log, BisType, OperLogInfo};
use crate::common::pglock::service::DbLock;
use crate::common::result::ResultData;
use crate::common::state::AppState;
use crate::common::token::AdminToken;
use crate::common::util::ip_util::IPUtil;
use crate::common::{consts, Response, ServerResult};
use actix_web::web::Data;
use actix_web::HttpRequest;
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;
use tokio::task_local;

task_local! {
    static ADMIN_CTX: AdminContext;
}

/// 管理后台 Context
#[derive(Clone)]
pub struct AdminContext {
    token: Option<AdminToken>,
}

impl AdminContext {
    pub fn new(token: Option<AdminToken>) -> Self {
        Self { token }
    }

    pub fn token_or_err(&self) -> ServerResult<AdminToken> {
        if let Some(v) = self.token {
            return Ok(v);
        }
        Err(ServerError::TokenError)
    }

    pub fn token_or_none(&self) -> Option<AdminToken> {
        self.token
    }
}

pub struct LockParam {
    key: String,
    wait: u64,
    keep: u64,
}

impl LockParam {
    pub fn new(key: String, wait: u64, keep: u64) -> Self {
        Self { key, wait, keep }
    }

    pub fn new_def(key: String) -> Self {
        Self {
            key,
            wait: 30,
            keep: 30,
        }
    }
}

/// 后台管理请求包装器
pub struct AdminHandler<Param, Res, F, Fut>
where
    Param: Serialize,
    Res: Serialize,
    F: FnOnce(AdminContext, Arc<AppState>, Param) -> Fut,
    Fut: Future<Output = ServerResult<Option<Res>>>,
{
    request: HttpRequest,
    param: Param,
    bis_type: Option<BisType>,
    must_login: bool,
    must_access: bool,
    lock: Option<LockParam>,
    lock_user: bool,
    action: Option<F>,
}

#[allow(dead_code)]
impl<Param, Res, F, Fut> AdminHandler<Param, Res, F, Fut>
where
    Param: Serialize,
    Res: Serialize,
    F: FnOnce(AdminContext, Arc<AppState>, Param) -> Fut,
    Fut: Future<Output = ServerResult<Option<Res>>> + Sized,
{
    pub fn of(request: HttpRequest, param: Param) -> Self {
        Self {
            request,
            param,
            bis_type: None,
            must_login: true,
            must_access: true,
            lock: None,
            lock_user: false,
            action: None,
        }
    }

    pub fn action(mut self, action: F) -> Self {
        self.action = Some(action);
        self
    }

    pub fn log_info(mut self, log: BisType) -> Self {
        self.bis_type = Some(log);
        self
    }

    pub fn skip_login(mut self) -> Self {
        self.must_login = false;
        self
    }

    pub fn use_refresh_token(mut self) -> Self {
        self.must_access = false;
        self
    }

    pub fn locked<LockF>(mut self, func: LockF) -> Self
    where LockF: FnOnce(&Param) -> Option<LockParam>
    {
        self.lock = func(&self.param);
        self
    }

    pub fn locked_key<LockF>(mut self, func: LockF) -> Self
    where LockF: FnOnce(&Param) -> String
    {
        let key = func(&self.param);
        self.lock = Some(LockParam::new_def(key));
        self
    }

    pub fn locked_str_key(mut self, key: &'static str) -> Self
    {
        self.lock = Some(LockParam::new_def(key.to_string()));
        self
    }

    pub fn lock_user(mut self) -> Self {
        self.lock_user = true;
        self
    }

    pub async fn run(self) -> Response<Res> {
        let action = self.action.ok_or(BusinessError("action is none"))?;
        
        let state = match self.request.app_data::<Data<AppState>>() {
            None => return Ok(ResultData::error("No State Data")),
            Some(v) => v.clone(),
        }.into_inner();

        // 读取Token
        let token_str = get_token_str(&self.request);
        let token = AdminTokenVerifier::new(token_str, self.must_login, self.must_access).verify().await?;

        // Context
        ADMIN_CTX.scope(AdminContext::new(token), async move {
            let context = ADMIN_CTX.get();
            let token = context.token_or_none();

            // 执行函数
            let action = async {
                // 操作日志
                let bis_type = if let Some(t) = self.bis_type {
                    t
                } else {
                    let rs = action(context, state.clone(), self.param).await?;
                    return Ok(rs);
                };

                // 保存日志
                let req_param = get_param_str(&self.param);
                // 计算耗时
                let start = Instant::now();
                let do_action = action(context, state.clone(), self.param).await;
                let exec_time = start.elapsed();

                // 是否执行成功
                let success = do_action.is_ok();
                // 错误信息
                let mut err_msg = String::new();
                let result = match do_action {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        err_msg.push_str(format!("{}", e).as_str());
                        Err(e)
                    }
                };

                // 日志信息
                let log_info = OperLogInfo {
                    title: bis_type.to_str(),
                    bis_type,
                    method: self.request.method().as_str().to_uppercase(),
                    uri: self.request.uri().path().to_string(),
                    exec_time: exec_time.as_millis() as i64,
                    req_ip: IPUtil::get_ip(&self.request),
                    req_param,
                    oper_user: token.map(|t| t.account_id()),
                    success,
                    error_msg: err_msg,
                };
                // 保存日志
                process_log(&state, log_info).await;

                result
            };

            let result = if let Some(param) = self.lock {
                let key = if self.lock_user && !param.key.is_empty() && let Some(token) = token {
                    format!("{}:u:{}", param.key, token.account_id())
                } else {
                    param.key
                };

                let lock_key = format!("{}:lock:{}", consts::PREFIX_KEY, key);
                let lock = DbLock::new(&state.db, &lock_key);
                if let Some(guard) = lock.acquire(param.wait, param.keep).await? {
                    let fut_result = action.await;
                    lock.release(guard).await?; // 释放锁
                    fut_result
                } else {
                    Err(BusinessError("系统错误, 请稍后再试"))
                }
            } else {
                action.await
            };
            ResultData::success(result?)
        }).await
    }
}

fn get_param_str<P: Serialize>(param: &P) -> String {
    let req_param = serde_json::to_string(param).unwrap_or_default();
    if req_param == "null" {
        String::new()
    } else {
        req_param
    }
}

fn get_token_str(req: &HttpRequest) -> String {
    req.headers().get(consts::TOKEN_HEADER)
        .map(|v| v.to_str().unwrap_or("").to_string())
        .unwrap_or(String::new())
}


struct AdminTokenVerifier {
    token_str: String,
    must_login: bool,
    must_access: bool,
}

impl AdminTokenVerifier {
    fn new(token_str: String, must_login: bool, must_access: bool) -> Self {
        Self { token_str, must_login, must_access }
    }

    async fn verify(&self) -> ServerResult<Option<AdminToken>> {
        let token = AdminToken::parse_str(&self.token_str);
        if let Some(token) = token {
            if self.must_access != token.is_access() {
                return Err(ServerError::TokenError);
            }
            return Ok(Some(token));
        }

        if self.must_login {
            return Err(ServerError::TokenError);
        }
        Ok(None)
    }
}
