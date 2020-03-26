pub mod buffer;
pub mod attribute;

use crate::vga::buffer::Buffer;

pub fn write(data: &str) -> usize {
    return Buffer::from_str(data).write();
}
