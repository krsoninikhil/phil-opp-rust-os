#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use phil_opp_rust_os::{exit_qemu, serial_println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");  // to detect as failed in case of panic

    serial_println!("{}", info);
    unsafe { exit_qemu(); }
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("ok");

    unsafe { exit_qemu(); }
    loop {}
}
