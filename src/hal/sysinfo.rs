use std::io;

use systemstat::{System, Platform};

pub fn get_arch() -> io::Result<String> {
    Ok(platforms::TARGET_ARCH.as_str().to_string())
}

pub fn get_os() -> io::Result<String> {
    Ok(platforms::TARGET_OS.as_str().to_string())
}

pub fn get_os_release() -> io::Result<String> {
    return match sys_info::os_release() {
        Ok(os_release) => Ok(os_release),
        Err(err) => { Err(io::Error::new(io::ErrorKind::Other, err.to_string())) },
    };
}

pub fn get_cpu_num() -> io::Result<u32> {
    return match sys_info::cpu_num() {
        Ok(cpu_num) => Ok(cpu_num),
        Err(err) => { Err(io::Error::new(io::ErrorKind::Other, err.to_string())) },
    };
}

pub fn get_cpu_speed() -> io::Result<u64> {
    return match sys_info::cpu_speed() {
        Ok(cpu_num) => Ok(cpu_num),
        Err(err) => { Err(io::Error::new(io::ErrorKind::Other, err.to_string())) },
    };
}

pub fn get_storage_total() -> io::Result<u64> {
    return match sys_info::disk_info() {
        Ok(cpu_num) => Ok(cpu_num.total),
        Err(err) => { Err(io::Error::new(io::ErrorKind::Other, err.to_string())) },
    };
}

pub fn get_storage_free() -> io::Result<u64> {
    return match sys_info::disk_info() {
        Ok(cpu_num) => Ok(cpu_num.free),
        Err(err) => { Err(io::Error::new(io::ErrorKind::Other, err.to_string())) },
    };
}

pub fn get_uptime() -> io::Result<String> {
    let sys = System::new();
    return match sys.uptime() {
        Ok(uptime) => Ok(format!("{:?}", uptime)),
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
    }
}

pub fn get_temperature() -> io::Result<f32> {
    let sys = System::new();
    return match sys.cpu_temp() {
        Ok(cpu_temp) => Ok(cpu_temp),
        Err(err) => Err(err),
    }
}
