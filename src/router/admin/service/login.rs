use crate::common::error::ServerError::BusinessError;
use crate::common::state::AppState;
use crate::common::token::AdminToken;
use crate::common::util::ip_util::IPUtil;
use crate::common::util::pwd_util::PasswordUtil;
use crate::common::util::vue_util::VueEncryptUtil;
use crate::common::ServerResult;
use crate::database::dao::sys_account::SysAccountDao;
use crate::router::admin::vo::login::{LoginReq, LoginResp, UpdatePwdReq};
use std::sync::Arc;

pub struct LoginService {
    state: Arc<AppState>,
}

impl LoginService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// 登录
    pub async fn login(&self, req: LoginReq, ip: String) -> ServerResult<LoginResp> {
        IPUtil::admin_check_ip_limit(&self.state, &ip).await?;

        let origin_pwd = VueEncryptUtil::client_decrypt(&req.password).unwrap_or("".to_string());
        let account = match SysAccountDao::new(&self.state.db).find_by_username(&req.username).await? {
            None => {
                IPUtil::admin_set_ip_limit(&self.state, &ip, req.username, origin_pwd).await?;
                return Err(BusinessError("账号密码不正确"));
            }
            Some(v) => v,
        };
        if !PasswordUtil::verify(&origin_pwd, &account.password) {
            IPUtil::admin_set_ip_limit(&self.state, &ip, req.username, origin_pwd).await?;
            return Err(BusinessError("账号密码不正确"));
        }
        if !account.status {
            return Err(BusinessError("账号已被禁用, 请联系管理员"));
        }

        Ok(LoginResp {
            access_token: AdminToken::access(account.id).to_jwt_str(),
            refresh_token: AdminToken::refresh(account.id).to_jwt_str()
        })
    }

    /// 刷新Token
    pub async fn refresh(&self, token: AdminToken) -> ServerResult<LoginResp> {
        let account_id = token.account_id();

        Ok(LoginResp {
            access_token: AdminToken::access(account_id).to_jwt_str(),
            refresh_token: AdminToken::refresh(account_id).to_jwt_str()
        })
    }

    /// 退出
    pub async fn logout(&self, _: AdminToken) -> ServerResult<()> {
        Ok(())
    }

    /// 修改密码
    pub async fn update_pwd(&self, token: AdminToken, param: UpdatePwdReq) -> ServerResult<()> {
        let old_pwd = VueEncryptUtil::client_decrypt(&param.old_pwd)?;

        let account = token.require_account(&self.state).await?;
        if !PasswordUtil::verify(&old_pwd, &account.password) {
            return Err(BusinessError("原密码不正确"));
        }

        let new_pwd = VueEncryptUtil::client_decrypt(&param.new_pwd)?;
        PasswordUtil::check_admin_len(&new_pwd)?;

        // 修改密码
        SysAccountDao::new(&self.state.db).update_pwd(account.id, &new_pwd).await?;
        Ok(())
    }

}
