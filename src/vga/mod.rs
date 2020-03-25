pub mod cell;
pub mod attribute;
pub mod write;

use crate::vga::cell::Cell;

/// Write a string to the VGA buffer
pub fn write_str(data: &str) {
    let mut idx = 0;

    for character in data.bytes() {
        Cell::write(Cell::new().with_character(character as char), idx);

        idx += 2;
    }
}

// FIXME: Add write!() macro

#[allow(dead_code)] // FIXME
pub fn write_bytes(data: &[u8]) {
    let mut idx = 0;

    for character in data {
        Cell::write(Cell::new().with_character(*character as char), idx);

        idx += 2;
    }
}
