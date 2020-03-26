use crate::vga::cell::Cell;

const BUF_H: usize = 25;
const BUF_W: usize = 80;

pub struct Buffer {
    position: usize,
    capacity: usize,

    data: [Cell; BUF_W * BUF_H],
}

impl Buffer {
    pub fn new() -> Buffer {
        let new_b = Buffer {
            position: 0,
            capacity: BUF_W * BUF_H,
            data: [Cell::new(); BUF_W * BUF_H],
        };

        new_b
    }

    pub fn from_str(data: &str) -> Buffer {
        let mut new_b = Buffer::new();

        for character in data.chars() {
            new_b.append(character);
        }

        new_b
    }

    pub fn append(&mut self, character: char) {
        self.data[self.position].set_character(character);
        self.position += 2;
    }

    pub fn write(&self) -> usize {
        let mut count = 0;

        for cell in self.data.iter() {
            cell.write(count);
            count += 1;
        }

        count
    }

    pub fn reset(&mut self) -> &Buffer {
        for i in (0..self.capacity) {
            self.data[i].set_character('\0');
        }

        self
    }

    pub fn flush(&mut self) -> &Buffer {
        self.write();
        self.reset();

        self
    }

    pub fn new_line(&mut self) -> &Buffer {
        //FIXME Add logic

        self
    }
}
