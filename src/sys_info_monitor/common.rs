// https://docs.rs/sysinfo/0.26.2/sysinfo/
#![allow(dead_code)]
use sysinfo::{DiskExt, System, SystemExt,ProcessExt,NetworkExt};
use log::{error, info, warn};
use log4rs;
// TODO: 正确获取 读写性能
// TODO:报错的正确处理
// 监控系统性能，用于方便控制本程序的读写IO，网络IO，CPU等，剩余磁盘空间不能占用过多，

pub fn init_sys_instance()->System{
    System::new_all()
}

pub fn sys_refresh(sys:&mut System){
    sys.refresh_all();
}

pub fn print_sys_info(){
    let mut sys = System::new_all();
    sys.refresh_all();
    print_disk(&sys);
    print_memory(&sys);
    match print_this_process(&sys){
        Err(e)=>error!("print_this_process err:{}",e),
        _=>()
    }
}

// 打印磁盘信息
pub fn print_disk(sys:&System){
    // We display all disks' information:
    for disk in sys.disks() {
        info!("disk {:?},available space:{} MB", disk.mount_point(),byte_to_million_byte(disk.available_space()));
    }
}

// 打印内存信息
pub fn print_memory(sys:&System){
    // RAM and swap information:
    info!("system total memory: {} MB", byte_to_million_byte(sys.total_memory()));
    info!("system used memory : {} MB", byte_to_million_byte(sys.used_memory()));
    info!("system total swap  : {} MB", byte_to_million_byte(sys.total_swap()));
    info!("system used swap   : {} MB", byte_to_million_byte(sys.used_swap()));
}

// 打印当前进程的信息
pub fn print_this_process(sys:&System)->Result<&str,&str>{
    let pid = sysinfo::get_current_pid()?;
    let process =  sys.process(pid).ok_or("pid not find.")?;

    let disk_usage = process.disk_usage();
    info!("memory use：{} MB,CUP use：{:.2}%",  
        byte_to_million_byte(process.memory()),
        process.cpu_usage()
    );
    info!("disk read：{} B,write: {} B ",disk_usage.read_bytes,disk_usage.written_bytes);
    return  Ok("");
}

// 打印网络状态
pub fn print_network(sys:&System){
    let networks = sys.networks();
    for (interface_name, network) in networks {
        info!("network {} received：{} B,send:{} B", 
            interface_name,
            network.received(),
            network.transmitted()
        );
    }
}

pub fn byte_to_million_byte(val:u64)->u64{
    return val/1024/1024;
}