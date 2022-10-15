mod sys_info_monitor;
use tokio::time;
use sys_info_monitor::common;
use log::{error, info, warn};
use log4rs;

// TODO: 日志打印记录，将系统状态 输出到文件里。
// https://docs.rs/log/latest/log/
// https://zhuanlan.zhihu.com/p/104921298
// TODO: 截图功能


#[tokio::main]
async fn main(){
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();


    let mut interval = time::interval(time::Duration::from_secs(2));
    let mut sys = common::init_sys_instance();
    common::print_disk(&sys);
    common::print_memory(&sys);
    loop {
        info!("--refresh--");

        interval.tick().await;
        common::sys_refresh(&mut sys);
        common::print_network(&mut sys);
        common::print_this_process(&sys).ok();
    }
}