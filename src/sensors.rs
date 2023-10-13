
use sysinfo::{ComponentExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
pub fn sys_test() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }

    println!("=> components:");
    for component in sys.components() {
        //println!("{:?}", component);
        println!("{}: {}", component.label(), component.temperature());
    }
    println!("=> system:");
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory: {} bytes", sys.used_memory());
    println!("total swap: {} bytes", sys.total_swap());
    println!("used swap: {} bytes", sys.used_swap());

    println!("System name: {:?}", sys.name());
    println!("System kernel version: {:?}", sys.kernel_version());
    println!("System OS version: {:?}", sys.os_version());
    println!("System host name: {:?}", sys.host_name());

    println!("NB CPUs: {}", sys.cpus().len());

    /*    for (pid, process) in sys.processes() {
            println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        }*/
}

pub fn get_disk_usage() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }
}