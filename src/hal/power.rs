use std::io;

pub fn shutdown() -> io::Result<()> {
    system_shutdown::shutdown()
}

pub fn reboot() -> io::Result<()> {
    system_shutdown::reboot()
}
