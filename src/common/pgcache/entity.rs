pub mod cache {
    use sea_orm::entity::prelude::*;
    use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

    pub type SysCacheActiveModel = ActiveModel;
    pub type SysCacheModel = Model;
    pub type SysCacheEntity = Entity;
    pub type SysCacheColumn = Column;

    #[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "sys_cache")]
    pub struct Model {
        #[sea_orm(primary_key, column_name = "id")]
        pub id: Uuid,
        #[sea_orm(column_name = "key")]
        pub key: String,
        #[sea_orm(column_name = "cache")]
        pub cache: String, // 缓存值
        #[sea_orm(column_name = "create_time")]
        pub create_time: DateTime, // 创建时间
        #[sea_orm(column_name = "expire_time")]
        pub expire_time: DateTime, // 过期时间
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod counter {
    use sea_orm::entity::prelude::*;
    use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

    pub type SysCounterActiveModel = ActiveModel;
    pub type SysCounterModel = Model;
    pub type SysCounterEntity = Entity;
    pub type SysCounterColumn = Column;

    #[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "sys_counter")]
    pub struct Model {
        #[sea_orm(primary_key, column_name = "id")]
        pub id: Uuid,
        #[sea_orm(column_name = "key")]
        pub key: String,
        #[sea_orm(column_name = "counter")]
        pub counter: i64, // 缓存值
        #[sea_orm(column_name = "create_time")]
        pub create_time: DateTime, // 创建时间
        #[sea_orm(column_name = "expire_time")]
        pub expire_time: DateTime, // 过期时间
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}