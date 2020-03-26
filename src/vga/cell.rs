use crate::vga::attribute::CellAttribute;
use crate::vga::write::write_data;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    attribute: CellAttribute,
    character: char,
}

impl Cell {
    pub fn new() -> Cell {
        let new_cell = Cell {
            attribute: CellAttribute::new(),
            character: '\0',
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

    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }

    pub fn set_attribute(&mut self, attribute: CellAttribute) {
        self.attribute = attribute;
    }

    pub fn write(&self, index: usize) {
        write_data(self.character as u8, index);
        self.attribute.write(index + 1);
    }

    pub fn reset(&mut self) {
        self.attribute.reset();
        self.character = '\0';
    }
}

