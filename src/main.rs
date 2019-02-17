#![no_std]  // don't link std library as we won't have it
#![no_main]  // don't call main as we need to define our own entry point

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga_buffer;

// called on panic, required because std is not linked and it won't
// compile w/o it
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

// entry point function, since on linux, linker looks for function
// named `_start` by default, on macOS, linker looks for `main`
// function; so change this function name to `main` if compiling on
// macOS; to preserve the function name, name mangling needs to
// disabled for this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_byte(b'H');
    vga_buffer::WRITER.lock().write_string("ello ");
    vga_buffer::WRITER.lock().write_str("World! \n").unwrap();
    write!(vga_buffer::WRITER.lock(), "Some numbers {}, {} \n", 42, 1.0/3.0).unwrap();
    print!("This is printed using {} macro.\n", "print!");
    println!("This is printed using {} macro. New line auto added.", "println!");

    loop {}
}
