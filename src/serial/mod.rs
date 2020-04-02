mod assembly;

// FIXME: Needed ?
pub fn syscall_write(data: &[u8], count: usize) {
    for i in 0..count {
        assembly::outb(assembly::COM1, data[i]);
    }
}

pub fn init_serial(port: u16) {
    /* We initialize the Baude Rate of the port to 38400 bps */
    assembly::outb(port + assembly::DLL_OFF, 0x3);
    assembly::outb(port + assembly::DLH_OFF, 0x0);
}

pub fn init_com1() {
    init_serial(assembly::COM1);
}

pub fn write_str(data: &str) {
    for byte in data.bytes() {
        if byte == '\n' as u8 {
            assembly::outb(assembly::COM1, '\r' as u8);
            assembly::outb(assembly::COM1, '\n' as u8);
        } else {
            assembly::outb(assembly::COM1, byte);
        }
    }
}
