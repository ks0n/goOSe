use crate::vga::attribute::Attribute;
use core::fmt;

const VGA_BUF_W: usize = 80;
const VGA_BUF_H: usize = 25;
const VGA_AREA: usize = VGA_BUF_H * VGA_BUF_W;

static VGA_BUFFER_ADDR: usize = 0xb8000;

fn write_byte(data: u8, index: usize) {
    unsafe {
        core::ptr::write_volatile((VGA_BUFFER_ADDR + index) as *mut u8, data);
    }
}

pub struct Buffer {
    size: usize,
    characters: [u8; VGA_AREA],
    attributes: [Attribute; VGA_AREA],
}

impl Buffer {
    pub fn new() -> Buffer {
        let new_buf = Buffer {
            size: 0,
            characters: [0x0; VGA_AREA],
            attributes: [*Attribute::new().with_fg_color(0xa); VGA_AREA],
        };

        new_buf
    }

    pub fn append_str(&mut self, data: &str) -> &Buffer {
        let bytes = data.as_bytes();

        for i in 0..data.len() {
            self.append(bytes[i]);
        }

        self
    }

    pub fn append(&mut self, data: u8) -> &Buffer {
        match data {
            // FIXME: Fix 'magical' values
            invalid_byte if invalid_byte >= 0x7e => {
                self.append('?' as u8);
            }
            b'\n' => {
                self.new_line();
            }
            _ => {
                self.characters[self.size] = data;
                self.size += 1;
            }
        };

        self
    }

    pub fn new_line(&mut self) -> &Buffer {
        // FIXME: Fix formula
        let mut cells_to_fill = ((VGA_BUF_W - (self.size % VGA_BUF_W)) / 2);
        cells_to_fill += if cells_to_fill % 2 == 0 { 1 } else { 0 };

        for i in 0..cells_to_fill {
            self.append(0);
        }

        self.size += cells_to_fill;

        self
    }

    pub fn reset(&mut self) -> &Buffer {
        for i in 0..self.size {
            self.characters[i] = 0x0;
            self.attributes[i] = Attribute::new();
        }

        self
    }

    pub fn write(&self) -> usize {
        let mut idx = 0;

        for tuple in self.characters.iter().zip(self.attributes.iter()) {
            let (symbol, attr) = tuple;

            write_byte(*symbol, idx);
            write_byte(attr.get_representation(), idx + 1);

            idx += 2;
        }

        return idx;
    }

    pub fn flush(&mut self) -> usize {
        let written = self.write();

        self.reset();

        return written;
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.append_str(s);
        self.write();

        Ok(())
    }
}
