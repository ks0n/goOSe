pub mod cell;
pub mod attribute;
pub mod write;

use crate::vga::cell::Cell;

/// Write a string to the VGA buffer
pub fn write(data: &str) -> u32 {
    let mut idx = 0;

    for character in data.bytes() {
        Cell::write(Cell::new().with_character(character as char), idx);

        idx += 2;
    }

    return idx;
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
