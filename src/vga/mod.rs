pub mod cell;
pub mod attribute;
pub mod write;
pub mod buffer;

use crate::vga::buffer::Buffer;

/// Write a string to the VGA buffer
pub fn write(data: &str) -> usize {
    let vga_buffer = Buffer::from_str(data);

    return vga_buffer.write();
}

mod tests {
    // FIXME: Add custom test framework that does not depend on `test`
    /*
    use super::*;

    #[cfg(test)]
    fn print_size() {
        assert_eq!(print("Hey"), 3);
    }
    */
}
