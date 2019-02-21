#![cfg_attr(not(test), no_std)]  // don't link std library as we won't have it
#![cfg_attr(not(test), no_main)]  // don't call main as we need to define our own entry point
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use core::fmt::Write;

use phil_opp_rust_os::*;

// called on panic, required because std is not linked and it won't
// compile w/o it
#[cfg(not(test))]  // don't include while test flag is set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

// entry point function, since on linux, linker looks for function
// named `_start` by default, on macOS, linker looks for `main`
// function; so change this function name to `main` if compiling on
// macOS
#[cfg(not(test))]
#[no_mangle]  // to preserve the function name, name mangling needs to disabled for this function
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_byte(b'H');
    vga_buffer::WRITER.lock().write_string("ello ");
    vga_buffer::WRITER.lock().write_str("World! \n").unwrap();

    write!(vga_buffer::WRITER.lock(), "Some numbers {}, {} \n", 42, 1.0/3.0).unwrap();
    print!("This is printed using {} macro.\n", "print!");
    println!("This is printed using {} macro. New line auto added.", "println!");

    serial::SERIAL1.lock().write_str("Kirk to Bridge:\n").expect("Printing to serial failed");
    serial_print!("This is printed using {} macro\n", "serial_print");
    serial_println!("This is printed using {} macro", "serial_println");

    interrupts::init_idt();

    x86_64::instructions::int3();
    println!("It did not crash!");

    loop {}
}
