#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use phil_opp_rust_os::{exit_qemu, serial_println, interrupts};

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
    interrupts::init_idt();  // load IDT
    x86_64::instructions::int3();  // cause breakpoint exception
    serial_println!("ok");  // if this executes, exception has been handled correctly

    unsafe { exit_qemu(); }
    loop {}
}
