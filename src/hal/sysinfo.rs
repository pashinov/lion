use std::io;

use systemstat::{System, Platform};

pub fn get_temperature() -> io::Result<f32> {
    let sys = System::new();
    return match sys.cpu_temp() {
        Ok(cpu_temp) => Ok(cpu_temp),
        Err(err) => Err(err),
    }
}

pub fn get_uptime() -> io::Result<String> {
    let sys = System::new();
    return match sys.uptime() {
        Ok(uptime) => Ok(format!("{:?}", uptime)),
        Err(err) => Err(err),
    }
}

pub fn get_boot_time() -> io::Result<String> {
    let sys = System::new();
    return match sys.boot_time() {
        Ok(boot_time) => Ok(format!("{}", boot_time)),
        Err(err) => Err(err),
    }
}

pub fn get_os_info() -> io::Result<String> {
    let os_type = match sys_info::os_type() {
        Ok(os_type) => os_type,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
        },
    };

    let os_release = match sys_info::os_release() {
        Ok(os_release) => os_release,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
        },
    };

    Ok(format!("{} {}", os_type, os_release))
}

pub fn get_cpu_info() -> io::Result<String> {
    let cpu_num = match sys_info::cpu_num() {
        Ok(cpu_num) => cpu_num,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
        },
    };

    let cpu_speed = match sys_info::cpu_speed() {
        Ok(cpu_speed) => cpu_speed,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
        },
    };

    Ok(format!("{} cores; {} MHz", cpu_num, cpu_speed))
}

pub fn get_disk_info() -> io::Result<String> {
    let disk_info = match sys_info::disk_info() {
        Ok(disk_info) => disk_info,
        Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
        },
    };

    Ok(format!("total {} KB; free {} KB", disk_info.total, disk_info.free))
}
