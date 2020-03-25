static VGA_BUFFER_ADDR: u32 = 0xb8000;

pub fn write_data(data: u8, index: u32) {
    unsafe {
        core::ptr::write_volatile((VGA_BUFFER_ADDR + index) as *mut u8, data);
    }
}
