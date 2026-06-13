use crate::common::pgq::dao::{SysQueueDelayMsgDao, SysQueueMsgDao};
use crate::common::pgq::entity::delay::SysQueueDelayMsgActiveModel;
use crate::common::pgq::entity::message::SysQueueMsgActiveModel;
use crate::common::pgq::MessageStatus;
use crate::common::{consts, ServerResult};
use chrono::{Duration, Local};
use sea_orm::{ConnectionTrait, Set};
use serde::Serialize;
use uuid::Uuid;

pub struct MessageSender<'d, C: ConnectionTrait> {
    conn: &'d C,
}

impl <'d, C: ConnectionTrait> MessageSender<'d, C> {
    pub fn new(conn: &'d C) -> Self {
        Self { conn }
    }

    /// 发送消息
    pub async fn send(&self, queue: &str, message: String) -> ServerResult<()> {
        let default_time = consts::get_default_time();
        let msg_dao = SysQueueMsgDao::new(self.conn);
        let msg = SysQueueMsgActiveModel {
            id: Set(Uuid::now_v7()),
            queue: Set(queue.to_string()),
            message: Set(message),
            status: Set(MessageStatus::Pending),
            publish_time: Set(Local::now().naive_local()),
            process_time: Set(default_time),
            finish_time: Set(default_time),
            error_detail: Set(String::new()),
        };
        msg_dao.insert(msg).await?;
        Ok(())
    }

    /// 发送JSON消息
    pub async fn send_json<T: Serialize>(&self, queue: &str, message: T) -> ServerResult<()> {
        let message = serde_json::to_string(&message)?;
        self.send(queue, message).await?;
        Ok(())
    }

    /// 发送延迟消息
    pub async fn send_delay(&self, queue: &str, message: String, delay: Duration) -> ServerResult<()> {
        let delay_time = Local::now().naive_local() + delay;
        let delay_msg_dao = SysQueueDelayMsgDao::new(self.conn);
        let msg = SysQueueDelayMsgActiveModel {
            id: Set(Uuid::now_v7()),
            queue: Set(queue.to_string()),
            message: Set(message),
            delay_time: Set(delay_time),
        };
        delay_msg_dao.insert(msg).await?;
        Ok(())
    }

    /// 发送延迟JSON消息
    pub async fn send_delay_json<T: Serialize>(&self, queue: &str, message: T, delay: Duration) -> ServerResult<()> {
        let message = serde_json::to_string(&message)?;
        self.send_delay(queue, message, delay).await?;
        Ok(())
    }

    /// 延迟1秒发送消息
    pub async fn send_delay_1s(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::seconds(1)).await
    }

    /// 延迟1秒发送消息
    pub async fn send_delay_json_1s<T: Serialize>(&self, queue: &str, message: T) -> ServerResult<()> {
        self.send_delay_json(queue, message, Duration::seconds(1)).await
    }

    /// 延迟3秒发送消息
    pub async fn send_delay_3s(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::seconds(3)).await
    }

    /// 延迟3秒发送消息
    pub async fn send_delay_json_3s<T: Serialize>(&self, queue: &str, message: T) -> ServerResult<()> {
        self.send_delay_json(queue, message, Duration::seconds(3)).await
    }
    /// 延迟5秒发送消息
    pub async fn send_delay_5s(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::seconds(5)).await
    }

    /// 延迟5秒发送消息
    pub async fn send_delay_json_5s<T: Serialize>(&self, queue: &str, message: T) -> ServerResult<()> {
        self.send_delay_json(queue, message, Duration::seconds(5)).await
    }

    /// 延迟10秒发送消息
    pub async fn send_delay_10s(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::seconds(10)).await
    }

    /// 延迟30秒发送消息
    pub async fn send_delay_30s(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::seconds(30)).await
    }

    /// 延迟1分钟发送消息
    pub async fn send_delay_1m(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::minutes(1)).await
    }

    /// 延迟2分钟发送消息
    pub async fn send_delay_2m(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::minutes(2)).await
    }

    /// 延迟3分钟发送消息
    pub async fn send_delay_3m(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::minutes(3)).await
    }

    /// 延迟3分钟发送消息
    pub async fn send_delay_5m(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::minutes(5)).await
    }

    /// 延迟10分钟发送消息
    pub async fn send_delay_10m(&self, queue: &str, message: String) -> ServerResult<()> {
        self.send_delay(queue, message, Duration::minutes(10)).await
    }

}

