static VGA_BUFFER_ADDR: usize = 0xb8000;

pub fn write_data(data: u8, index: usize) {
    unsafe {
        core::ptr::write_volatile((VGA_BUFFER_ADDR + index) as *mut u8, data);
    }
}
