#![no_std]  // don't link std library as we won't have it
#![no_main]  // don't call main as we need to define our own entry point

use core::panic::PanicInfo;

// called on panic, required because std is not linked and it won't
// compile w/o it
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// text to print on screen
static DISPLAY_TEXT: &[u8] = b"Hello World!";

// entry point function, since on linux, linker looks for function
// named `_start` by default, on macOS, linker looks for `main`
// function; so change this function name to `main` if compiling on
// macOS; to preserve the function name, name mangling needs to
// disabled for this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // raw pointer to VGA buffer, content of which will be displayed
    // on screen
    let vga_buffer = 0xb8000 as *mut u8;

    // update vga_buffer with desired bytes; each character cell of
    // this buffer consists of an ASCII byte and a color byte
    for (i, &byte) in DISPLAY_TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
