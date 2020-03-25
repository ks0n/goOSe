use crate::vga::write::write_data;

pub struct CellAttribute {
    blink: bool,
    bg_color: u8,
    fg_color: u8,
}

impl CellAttribute {
    pub fn new() -> CellAttribute {
        let new_ca = CellAttribute {
            blink: false,
            bg_color: 0x0,
            fg_color: 0xa,
        };

        new_ca
    }

    pub fn with_blink(&mut self, blink: bool) -> &CellAttribute {
        self.blink = blink;

        return self;
    }

    pub fn with_bg_color(&mut self, bg_color: u8) -> &CellAttribute {
        self.bg_color = bg_color;

        return self;
    }

    pub fn with_fg_color(&mut self, fg_color: u8) -> &CellAttribute {
        self.fg_color = fg_color;

        return self;
    }

    // FIXME: Add code
    pub fn get_u8_representation(&self) -> u8 {
        0xa
    }

    pub fn write(&self, index: u32) {
        write_data(self.get_u8_representation(), index);
    }
}
