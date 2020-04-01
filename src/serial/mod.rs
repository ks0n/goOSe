mod assembly;

// FIXME: Needed ?
pub fn syscall_write(data: &[u8], count: usize) {
    for i in 0..count {
        assembly::outb(assembly::COM1, data[i]);
    }
}

pub fn write_str(data: &str) {
    for byte in data.bytes() {
        assembly::outb(assembly::COM1, byte);
    }
}
