pub fn outb(addr: u16, byte: u8) {
    let addr = addr as *mut u8;
    unsafe {
        *addr = byte;
    }
}
