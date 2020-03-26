#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "utests_launch"]

mod panic_handler;
mod utest;
mod vga;

static GREET: &str = "Talk to me, Goose !\nWelcome to goOSe !\n\nAnd another one";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    utests_launch();

    // FIXME: Add multiple write support
    vga::write(GREET);
    vga::write("HeWörld");

    loop {}
}

mod tests {
    // FIXME: Add test for invalid characters: vga::write("HeWörld");
}
