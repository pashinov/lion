use log::{error};
use protobuf::Message;

use crate::hal;
use crate::lion;

pub fn handle_request(request: &String) -> String {
    let request: lion::Request = protobuf::parse_from_bytes(request.as_bytes()).unwrap();

    let mut response = lion::Response::new();
    response.command = request.command.clone();
    response.resource = request.resource.clone();
    response.status = lion::ResponseStatus::FAIL;

    match request.command {
        lion::CommandType::SET => {
            if request.get_resource().has_power() {
                match request.get_resource().get_power().field_type {
                    lion::PowerType::SHUTDOWN => {
                        if request.get_payload().has_bval() && request.get_payload().get_bval() {
                            match hal::power::shutdown() {
                                Ok(()) => {
                                    response.status = lion::ResponseStatus::OK;
                                },
                                Err(err) => {
                                    let mut payload = lion::PayloadType::new();
                                    payload.set_sval(err.to_string());
                                    response.set_payload(payload);
                                    error!("Failed to power off of board");
                                },
                            }
                        }
                    },
                    lion::PowerType::REBOOT => {
                        if request.get_payload().has_bval() && request.get_payload().get_bval() {
                            match hal::power::reboot() {
                                Ok(()) => {
                                    response.status = lion::ResponseStatus::OK;
                                },
                                Err(err) => {
                                    let mut payload = lion::PayloadType::new();
                                    payload.set_sval(err.to_string());
                                    response.set_payload(payload);
                                    error!("Failed to reboot of board");
                                },
                            }
                        }
                    },
                }
            }
        },
        lion::CommandType::GET => {
            if request.get_resource().has_sysinfo() {
                match request.get_resource().get_sysinfo().field_type {
                    lion::SysInfoType::UPTIME => {
                        match hal::sysinfo::get_uptime() {
                            Ok(uptime) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(uptime);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get a uptime of board");
                            },
                        }
                    },
                    lion::SysInfoType::BOOT_TIME => {
                        match hal::sysinfo::get_boot_time() {
                            Ok(boot_time) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(boot_time);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get a boot time of board");
                            },
                        }
                    },
                    lion::SysInfoType::TEMPERATURE => {
                        match hal::sysinfo::get_temperature() {
                            Ok(temp) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_rval(temp as f64);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get a temperature of board");
                            },
                        };
                    },
                    lion::SysInfoType::OS_INFO => {
                        match hal::sysinfo::get_os_info() {
                            Ok(os_info) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(os_info);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get a os info of board");
                            },
                        };
                    },
                    lion::SysInfoType::CPU_INFO => {
                        match hal::sysinfo::get_cpu_info() {
                            Ok(cpu_info) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(cpu_info);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get a cpu info of board");
                            },
                        };
                    },
                    lion::SysInfoType::DISK_INFO => {
                        match hal::sysinfo::get_disk_info() {
                            Ok(disk_info) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(disk_info);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get a disk info of board");
                            },
                        };
                    },
                }
            } else {
                let mut payload = lion::PayloadType::new();
                payload.set_sval("Not supported".to_string());
                response.set_payload(payload);
                error!("Requested resource is not supported");
            }
        },
    }

    String::from_utf8(response.write_to_bytes().unwrap()).expect("Found invalid UTF-8")
}
