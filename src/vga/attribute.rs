#[derive(Copy, Clone)]
pub struct Attribute {
    blink: bool,
    bg_color: u8,
    fg_color: u8,
}

impl Attribute {
    pub fn new() -> Attribute {
        let new_attr = Attribute {
            blink: false,
            bg_color: 0x0,
            fg_color: 0xa,
        };

        new_attr
    }

    pub fn with_blink(&mut self, blink: bool) -> &Attribute {
        self.blink = blink;

        self
    }

    pub fn with_bg_color(&mut self, bg_color: u8) -> &Attribute {
        self.bg_color = bg_color;

        self
    }

    pub fn with_fg_color(&mut self, fg_color: u8) -> &Attribute {
        self.fg_color = fg_color;

        self
    }

    pub fn get_representation(&self) -> u8 {
        let mut repr: u8 =
            if self.blink {
                0b10000000
            } else {
                0b00000000
            };

        // FIXME: This erases the boolean set
        repr |= self.bg_color << 4;
        repr |= self.fg_color;

        repr
    }
}

mod tests {
    // FIXME:
}
