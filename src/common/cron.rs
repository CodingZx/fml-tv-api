use crate::common::{logger, ServerResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler};

#[async_trait]
pub trait CornTask: Clone + Sync + Send {
    fn name() -> &'static str;

    fn cron() -> &'static str;

    async fn execute(&self) -> ServerResult<()>;
}


pub struct CornManager {
    scheduler: JobScheduler,
    jobs: HashMap<String, Job>,
}

impl CornManager {
    pub async fn new() -> Self {
        let scheduler = JobScheduler::new().await.expect("定时任务调度器创建失败");
        Self {
            scheduler,
            jobs: HashMap::new(),
        }
    }

    pub fn add_task<C: CornTask + 'static>(&mut self, task: C) {
        let task_name = String::from(C::name());
        let task = Arc::new(task);
        let builder = JobBuilder::new()
            .with_timezone(chrono_tz::Asia::Shanghai)
            .with_cron_job_type()
            .with_schedule(C::cron())
            .unwrap_or_else(|_| panic!("创建定时任务[{}]失败, 表达式错误", &task_name));
        let s = builder.with_run_async(Box::new(move |_uuid, _l| {
            let task = Arc::clone(&task);
            Box::pin(async move {
                if let Err(err) = task.execute().await {
                    logger::error!("定时任务[{}]执行发生错误, 错误信息: {}", C::name(), err)
                }
            })
        })).build().unwrap_or_else(|e| panic!("创建定时任务[{}]失败, 错误信息: {}", &task_name, e));

        if self.jobs.contains_key(&task_name) {
            panic!("定时任务[{}]重复, 请确认任务名称是否正确!", &task_name);
        }
        self.jobs.insert(task_name, s);
    }

    pub async fn start(self) {
        for (name, job) in self.jobs {
            self.scheduler
                .add(job)
                .await
                .unwrap_or_else(|e| panic!("添加定时任务[{name}]失败, 错误信息: {e}"));
            logger::info!("添加定时任务[{}]成功", name);
        }
        self.scheduler.start().await.expect("定时任务启动失败");
    }
}