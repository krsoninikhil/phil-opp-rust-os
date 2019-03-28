#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

use phil_opp_rust_os::{exit_qemu, serial_println, gdt};

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
#[allow(unconditional_recursion)]
pub extern "C" fn _start() -> ! {
    gdt::init();
    init_idt();

    // trigger stack overflow
    fn overflow_stack() {
        overflow_stack();
    }
    overflow_stack();

    serial_println!("failed");
    serial_println!("No exception occured");  // if this executes, exception has been handled correctly

    unsafe { exit_qemu(); }
    loop {}
}

// IDT needs to be redefined as we need to have a different handler
// for testing and existing one is not mutable
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64
) {
    serial_println!("ok");
    unsafe { exit_qemu(); }
    loop {}
}
