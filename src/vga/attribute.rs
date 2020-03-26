use crate::vga::write::write_data;

static DEFAULT_BG_COLOR: u8 = 0x0;
static DEFAULT_FG_COLOR: u8 = 0xa;
static DEFAULT_BLINK: bool = false;

#[derive(Debug, Clone, Copy)]
pub struct CellAttribute {
    blink: bool,
    bg_color: u8,
    fg_color: u8,
}

impl CellAttribute {
    pub fn new() -> CellAttribute {
        let new_ca = CellAttribute {
            blink: DEFAULT_BLINK,
            bg_color: DEFAULT_BG_COLOR,
            fg_color: DEFAULT_FG_COLOR,
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

    pub fn write(&self, index: usize) {
        write_data(self.get_u8_representation(), index);
    }

    pub fn reset(&mut self) {
        self.blink = DEFAULT_BLINK;
        self.bg_color = DEFAULT_BG_COLOR;
        self.fg_color = DEFAULT_FG_COLOR;
    }
}
