pub mod message {
    use crate::common::pgq::MessageStatus;
    use sea_orm::entity::prelude::*;
    use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};

    pub type SysQueueMsgActiveModel = ActiveModel;
    pub type SysQueueMsgModel = Model;
    pub type SysQueueMsgEntity = Entity;
    pub type SysQueueMsgColumn = Column;

    #[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "sys_queue_msg")]
    pub struct Model {
        #[sea_orm(primary_key, column_name = "id")]
        pub id: Uuid,
        #[sea_orm(column_name = "queue")]
        pub queue: String, // 队列名称
        #[sea_orm(column_name = "message")]
        pub message: String, // 消息内容
        #[sea_orm(column_name = "status")]
        pub status: MessageStatus, // 消息状态
        #[sea_orm(column_name = "publish_time")]
        pub publish_time: DateTime, // 发布时间
        #[sea_orm(column_name = "process_time")]
        pub process_time: DateTime, // 处理时间
        #[sea_orm(column_name = "finish_time")]
        pub finish_time: DateTime, // 完成时间
        #[sea_orm(column_name = "error_detail")]
        pub error_detail: String, // 错误详情
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod delay {
    use sea_orm::entity::prelude::*;
    use sea_orm::prelude::DateTime;
    use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
    use uuid::Uuid;

    pub type SysQueueDelayMsgActiveModel = ActiveModel;
    pub type SysQueueDelayMsgEntity = Entity;
    pub type SysQueueDelayMsgColumn = Column;

    #[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "sys_queue_delay_msg")]
    pub struct Model {
        #[sea_orm(primary_key, column_name = "id")]
        pub id: Uuid,
        #[sea_orm(column_name = "queue")]
        pub queue: String, // 队列名称
        #[sea_orm(column_name = "message")]
        pub message: String, // 消息内容
        #[sea_orm(column_name = "delay_time")]
        pub delay_time: DateTime, // 发布时间
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}