use core::fmt::{Arguments, Write};

/*
macro_rules! print {
    ($($arg::tt)*) => ($crate::vga::macro_print());
}

macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg::tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn macro_print(args: fmt::Arguments) {
    vga::write_fmt(args).unwrap();
}
*/
