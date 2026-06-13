use sea_orm::{DeriveActiveEnum, EnumIter, FromQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod entity;
pub mod dao;
pub mod service;
pub mod consumer;

///  消息类型
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum MessageStatus {
    #[sea_orm(num_value = 0)]
    Pending,
    #[sea_orm(num_value = 1)]
    Processing,
    #[sea_orm(num_value = 2)]
    Success,
    #[sea_orm(num_value = -1)]
    Failed,
}

impl MessageStatus {
    pub fn value(&self) -> i32 {
        match self {
            MessageStatus::Pending => 0,
            MessageStatus::Processing => 1,
            MessageStatus::Success => 2,
            MessageStatus::Failed => -1,
        }
    }
    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(MessageStatus::Pending),
            1 => Some(MessageStatus::Processing),
            2 => Some(MessageStatus::Success),
            -1 => Some(MessageStatus::Failed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct MessageBody {
    id: Uuid,
    message: String,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct DelayMessageBody {
    id: Uuid,
    message: String,
    queue: String,
}


