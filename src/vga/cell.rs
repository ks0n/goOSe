use crate::vga::attribute::CellAttribute;
use crate::vga::write::write_data;

pub struct Cell {
    attribute: CellAttribute,
    character: char,
}

impl Cell {
    pub fn new() -> Cell {
        let new_cell = Cell {
            attribute: CellAttribute::new(),
            character: '0',
        };

        new_cell
    }

    pub fn with_character(&mut self, character: char) -> &Cell {
        self.character = character;

        return self;
    }

    pub fn with_attribute(&mut self, attribute: CellAttribute) -> &Cell {
        self.attribute = attribute;

        return self;
    }

    // FIXME: Add Result as return value ?
    pub fn write(&self, index: u32) {
        write_data(self.character as u8, index);
        self.attribute.write(index + 1);
    }
}

