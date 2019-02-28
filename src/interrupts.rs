// function with x86-interrupt calling convention doesn't work as of
// now if compiled for Windows target due to a bug in LLVM
// (`cargo test`)
#![cfg(not(windows))]

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use lazy_static::lazy_static;

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("Exception: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    println!("Exception: Double Fault\n{:#?}", stack_frame);
    loop {}
}
