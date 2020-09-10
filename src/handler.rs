use log::{error};
use protobuf::Message;

use crate::hal;
use crate::lion;

pub fn handle_request(request: &String) -> Vec<u8> {
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
                    lion::SysInfoType::ARCH => {
                        match hal::sysinfo::get_arch() {
                            Ok(arch) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(arch);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get ARCH");
                            },
                        }
                    },
                    lion::SysInfoType::OS => {
                        match hal::sysinfo::get_os() {
                            Ok(os) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(os);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get OS");
                            },
                        }
                    },
                    lion::SysInfoType::OS_RELEASE => {
                        match hal::sysinfo::get_os_release() {
                            Ok(os_release) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(os_release);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get OS Release");
                            },
                        }
                    },
                    lion::SysInfoType::CPU_NUM => {
                        match hal::sysinfo::get_cpu_num() {
                            Ok(cpu_num) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_uval(cpu_num);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get CPU numbers");
                            },
                        }
                    },
                    lion::SysInfoType::CPU_SPEED => {
                        match hal::sysinfo::get_cpu_speed() {
                            Ok(cpu_speed) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_uval(cpu_speed as u32);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get CPU speed");
                            },
                        }
                    },
                    lion::SysInfoType::STORAGE_TOTAL => {
                        match hal::sysinfo::get_storage_total() {
                            Ok(storage_total) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_uval(storage_total as u32);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get Storage Total space");
                            },
                        }
                    },
                    lion::SysInfoType::STORAGE_FREE => {
                        match hal::sysinfo::get_storage_free() {
                            Ok(storage_free) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_uval(storage_free as u32);
                                response.set_payload(payload);
                                response.status = lion::ResponseStatus::OK;
                            },
                            Err(err) => {
                                let mut payload = lion::PayloadType::new();
                                payload.set_sval(err.to_string());
                                response.set_payload(payload);
                                error!("Failed to get Storage Free space");
                            },
                        }
                    },
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
                                error!("Failed to get Uptime");
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
                                error!("Failed to get Temperature");
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

    response.write_to_bytes().unwrap()
}
