pub mod cell;
pub mod attribute;
pub mod write;

use crate::vga::cell::Cell;

/// Write a string to the VGA buffer
pub fn print(data: &str) -> u32 {
    let mut idx = 0;

    for character in data.bytes() {
        Cell::write(Cell::new().with_character(character as char), idx);

        idx += 2;
    }

    return idx;
}
