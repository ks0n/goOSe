static VGA_BUFFER_ADDR: u32 = 0xb8000;

pub fn write_bytes(data: &[u8]) {
    let mut idx = 0;

    for character in data {
        unsafe {
            /* Write the character on the VGA Buffer */
            core::ptr::write_volatile((VGA_BUFFER_ADDR + idx) as *mut u8, *character as u8);

            /* Set the color of the previous character */
            core::ptr::write_volatile((VGA_BUFFER_ADDR + idx + 1) as *mut u8, 0xa);
        }

        idx += 2;
    }
}
