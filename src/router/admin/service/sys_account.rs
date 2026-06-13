use crate::common::error::ServerError::{BusinessError, OptimisticLock};
use crate::common::model::{Page, Pager};
use crate::common::state::AppState;
use crate::common::token::AdminToken;
use crate::common::util::pwd_util::PasswordUtil;
use crate::common::util::vue_util::VueEncryptUtil;
use crate::common::ServerResult;
use crate::database::dao::sys_account::SysAccountDao;
use crate::database::model::sys_account::{SysAccountActiveModel, SysAccountColumn};
use crate::router::admin::vo::sys_account::{SysAccountDeleteReq, SysAccountListReq, SysAccountListResp, SysAccountResetPwdReq, SysAccountSaveReq, SysAccountStatusReq};
use chrono::Local;
use sea_orm::{ColumnTrait, Condition, NotSet, Set, TransactionTrait};
use std::sync::Arc;
use uuid::Uuid;

pub struct SysAccountService {
    state: Arc<AppState>,
}

impl SysAccountService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// 查询列表
    pub async fn list(&self, token: AdminToken, param: SysAccountListReq) -> ServerResult<Pager<SysAccountListResp>> {
        let mut conditions = Condition::all();
        let user_name = param.user_name.unwrap_or_default();
        if !user_name.is_empty() {
            conditions = conditions.add(SysAccountColumn::Username.contains(&user_name));
        }
        let real_name = param.real_name.unwrap_or_default();
        if !real_name.is_empty() {
            conditions = conditions.add(SysAccountColumn::RealName.contains(&real_name));
        }

        let current = token.require_account(&self.state).await?;
        if !current.super_admin {
            conditions = conditions.add(SysAccountColumn::SuperAdmin.ne(true));
        }
        conditions = conditions.add(SysAccountColumn::Deleted.eq(false));

        let page = Page::from(param.page, param.size);

        let (records, count) = SysAccountDao::new(&self.state.db)
            .find_page_list(page, conditions)
            .await?;

        let mut result = Vec::with_capacity(records.len());
        for account in records {
            result.push(SysAccountListResp::new(account));
        }

        Ok(Pager::new(result, count))
    }

    /// 保存账号
    pub async fn save(&self, token: AdminToken, param: SysAccountSaveReq) -> ServerResult<()> {
        let now = Local::now().naive_local();
        let dao = SysAccountDao::new(&self.state.db);
        let check_user_name = dao.find_by_username(&param.username).await?;

        let mut model = SysAccountActiveModel {
            id: NotSet,
            username: Set(param.username),
            real_name: Set(param.real_name),
            password: NotSet,
            status: NotSet,
            super_admin: NotSet,
            update_time: Set(now),
            update_user: Set(token.account_id()),
            ..Default::default()
        };
        match param.id {
            Some(id) => {
                if dao.find_undeleted_by_id(id).await?.is_none() {
                    return Err(BusinessError("ID错误, 数据不存在"));
                }
                // 修改
                if check_user_name.map(|r|r.id != id).unwrap_or(false) {
                    return Err(BusinessError("用户名重复"));
                }
                model.status = Set(true);
                model.version = Set(param.version + 1);

                let row = dao.update_by_version(model, id, param.version).await?;
                if row == 0 {
                    return Err(OptimisticLock);
                }
            }
            None => {
                if check_user_name.is_some() {
                    return Err(BusinessError("用户名重复"));
                }
                model.id = Set(Uuid::now_v7());
                model.super_admin = Set(false);
                model.version = Set(1);
                // 填充逻辑删除字段
                model.login_delete_created();

                let pwd = VueEncryptUtil::client_decrypt(&param.password.unwrap_or_default())?;
                PasswordUtil::check_admin_len(&pwd)?;

                model.password = Set(PasswordUtil::generate(&pwd)?);
                model.create_time = Set(now);
                model.create_user = Set(token.account_id());
                dao.insert(model).await?;
            }
        }

        Ok(())
    }

    /// 重置密码
    pub async fn reset_pwd(&self, token: AdminToken, param: SysAccountResetPwdReq) -> ServerResult<()> {
        let account = SysAccountDao::new(&self.state.db).find_undeleted_by_id(param.id).await?.ok_or(BusinessError("ID错误, 数据不存在"))?;

        let new_pwd = VueEncryptUtil::client_decrypt(&param.password)?;
        PasswordUtil::check_admin_len(&new_pwd)?;

        let update = SysAccountActiveModel {
            id: Set(account.id),
            password: Set(PasswordUtil::generate(&new_pwd)?),
            update_time: Set(Local::now().naive_local()),
            update_user: Set(token.account_id()),
            ..Default::default()
        };

        SysAccountDao::new(&self.state.db).update_by_version(update, param.id, param.version).await?;
        Ok(())
    }

    /// 修改状态
    pub async fn update_status(&self, token: AdminToken, param: SysAccountStatusReq) -> ServerResult<()> {
        let current = token.require_account(&self.state).await?;

        let account = SysAccountDao::new(&self.state.db).find_undeleted_by_id(param.id).await?.ok_or(BusinessError("ID错误, 数据不存在"))?;
        if !current.super_admin && account.super_admin {
            return Err(BusinessError("无法修改超级管理员状态"));
        }

        let model = SysAccountActiveModel {
            status: Set(param.status),
            update_user: Set(token.account_id()),
            update_time: Set(Local::now().naive_local()),
            version: Set(param.version + 1),
            ..Default::default()
        };
        let rows = SysAccountDao::new(&self.state.db).update_by_version(model, param.id, param.version).await?;
        if rows == 0 {
            return Err(OptimisticLock);
        }
        Ok(())
    }

    /// 删除
    pub async fn delete(&self, token: AdminToken, param: SysAccountDeleteReq) -> ServerResult<()> {
        let mut id_ver = Vec::new();
        {
            let account_dao = SysAccountDao::new(&self.state.db);
            for it in param.id_ver.into_iter() {
                let account = account_dao.find_undeleted_by_id(it.id).await?;
                if let Some (v) = account {
                    if v.super_admin {
                        return Err(BusinessError("无法删除超级管理员"));
                    }
                    id_ver.push(it);
                }
            }
        }

        self.state.db.transaction(|tx| {
            Box::pin(async move {
                let account_dao = SysAccountDao::new(tx);
                for it in id_ver {
                    let mut model = SysAccountActiveModel {
                        version: Set(it.version + 1),
                        ..Default::default()
                    };
                    model.login_deleted(token.account_id(), param.reason.clone());
                    let rows = account_dao.update_by_version(model, it.id, it.version).await?;
                    if rows == 0 {
                        return Err(OptimisticLock);
                    }
                }
                Ok(())
            })
        }).await?;
        Ok(())
    }
}
