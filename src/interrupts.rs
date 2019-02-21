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
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("Exception: Breakpoint\n{:#?}", stack_frame);
}
