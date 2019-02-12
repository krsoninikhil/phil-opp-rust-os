#![no_std]  // don't link std library as we won't have it
#![no_main]  // don't call main as we need to define our own entry point

use core::panic::PanicInfo;

// called on panic, required because std is not linked and it won't
// compile w/o it
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// entry point function, since on linux, linker looks for function
// named `_start` by default, on macOS, linker looks for `main`
// function; to preserve the function name, name mangling needs to
// disabled for this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // change this function name to `main` if compiling on macOS
    loop {}
}
