use log::{debug, error, info};

fn main() {
     // 初始化日志收集
    env_logger::init();
    info!("这是一条info日志");
    debug!("这是一条debug日志");
    error!("这是一条error日志");
}
