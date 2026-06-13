use crate::common::model::Page;
use crate::common::util::pwd_util::PasswordUtil;
use crate::common::ServerResult;
use crate::database::model::sys_account::*;
use chrono::Local;
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use uuid::Uuid;

pub struct SysAccountDao<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl<'d, C: ConnectionTrait> SysAccountDao<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 根据ID查询
    pub async fn find_by_id(&self, id: Uuid) -> ServerResult<Option<SysAccountModel>> {
        let account = SysAccountEntity::find_by_id(id)
            .one(self.conn)
            .await?;
        Ok(account)
    }

    /// 根据ID查询未删除账号
    pub async fn find_undeleted_by_id(&self, role_id: Uuid) -> ServerResult<Option<SysAccountModel>> {
        let role = SysAccountEntity::find()
            .filter(SysAccountColumn::Id.eq(role_id))
            .filter(SysAccountColumn::Deleted.eq(false))
            .one(self.conn)
            .await?;
        Ok(role)
    }

    /// Insert
    pub async fn insert(&self, model: SysAccountActiveModel) -> ServerResult<()> {
        SysAccountEntity::insert(model)
            .exec_without_returning(self.conn)
            .await?;
        Ok(())
    }

    /// 根据ID和版本修改
    pub async fn update_by_version(&self, record: SysAccountActiveModel, id: Uuid, version: i32) -> ServerResult<u64> {
        let res = SysAccountEntity::update_many()
            .set(record)
            .filter(SysAccountColumn::Id.eq(id))
            .filter(SysAccountColumn::Version.eq(version))
            .exec(self.conn)
            .await?;
        Ok(res.rows_affected)
    }

    /// 修改密码
    pub async fn update_pwd(&self, id: Uuid, pwd: &str) -> ServerResult<()> {
        // 修改密码
        let update = SysAccountActiveModel {
            id: Set(id),
            password: Set(PasswordUtil::generate(pwd)?),
            update_time: Set(Local::now().naive_local()),
            update_user: Set(id),

            ..Default::default()
        };
        SysAccountEntity::update(update)
            .exec(self.conn)
            .await?;
        Ok(())
    }

    /// 查询列表
    pub async fn find_page_list(&self, page: Page, condition: Condition) -> ServerResult<(Vec<SysAccountModel>, u64)> {
        let total = SysAccountEntity::find()
            .filter(condition.clone())
            .count(self.conn)
            .await?;
        if total == 0 {
            return Ok((Vec::new(), 0));
        }

        let list = SysAccountEntity::find()
            .filter(condition)
            .order_by_desc(SysAccountColumn::CreateTime)
            .offset(page.offset())
            .limit(page.size())
            .all(self.conn)
            .await?;

        Ok((list, total))
    }

    /// 查询账号
    pub async fn find_by_username(&self, username: &str) -> ServerResult<Option<SysAccountModel>> {
        let account = SysAccountEntity::find()
            .filter(SysAccountColumn::Username.eq(username))
            .filter(SysAccountColumn::Deleted.eq(false))
            .one(self.conn)
            .await?;
        Ok(account)
    }

}
