pub mod attribute;
pub mod buffer;
pub mod macros;

use crate::vga::buffer::Buffer;

pub fn write(buffer: &mut Buffer, data: &str) -> usize {
    return buffer.append_str(data).write();
}
