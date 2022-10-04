// https://docs.rs/sysinfo/0.26.2/sysinfo/

use sysinfo::{DiskExt, System, SystemExt,ProcessExt};
// 监控系统性能，用于方便控制本程序的读写IO，网络IO，CPU等，剩余磁盘空间不能占用过多，
pub fn print_sys_info(){
    let mut sys = System::new_all();
    sys.refresh_all();
    print_disk(&sys);
    print_memory(&sys);
    print_this_process(&sys).ok();

}

// 打印磁盘信息
fn print_disk(sys:&System){
    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?},available space:{} MB", disk.mount_point(),byte_to_million_byte(disk.available_space()));
    }
}

// 打印内存信息
fn print_memory(sys:&System){
    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} MB", byte_to_million_byte(sys.total_memory()));
    println!("used memory : {} MB", byte_to_million_byte(sys.used_memory()));
    println!("total swap  : {} MB", byte_to_million_byte(sys.total_swap()));
    println!("used swap   : {} MB", byte_to_million_byte(sys.used_swap()));
}

// 打印当前进程的信息
fn print_this_process(sys:&System)->Result<(),String>{
    let pid = sysinfo::get_current_pid()?;
    let process =  sys.process(pid).ok_or("没找到本序的Pid。")?;
    println!("{}，使用内存：{} MB,CUP：{}%",  process.name(),byte_to_million_byte(process.memory()),process.cpu_usage());
    return  Ok(());
}

fn byte_to_million_byte(val:u64)->u64{
    return val/1024/1024;
}