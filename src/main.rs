mod sys_info_monitor;
use tokio::time;
use sys_info_monitor::common;

#[tokio::main]
async fn main(){
    
    let mut interval = time::interval(time::Duration::from_secs(2));
    let mut sys = common::init_sys_instance();
    common::print_disk(&sys);
    common::print_memory(&sys);
    loop {
        interval.tick().await;
        common::sys_refresh(&mut sys);
        common::print_network(&mut sys);
        common::print_this_process(&sys).ok();
    }
}