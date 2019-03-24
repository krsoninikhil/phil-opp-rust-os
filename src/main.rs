#![cfg_attr(not(test), no_std)]  // don't link std library as we won't have it
#![cfg_attr(not(test), no_main)]  // don't call main as we need to define our own entry point
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use phil_opp_rust_os::*;

// called on panic, required because std is not linked and it won't
// compile w/o it
#[cfg(not(test))]  // don't include while test flag is set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    hlt_loop();
}

// entry point function, since on linux, linker looks for function
// named `_start` by default, on macOS, linker looks for `main`
// function; so change this function name to `main` if compiling on
// macOS
#[cfg(not(test))]
#[no_mangle]  // to preserve the function name, name mangling needs to disabled for this function
pub extern "C" fn _start() -> ! {
    print!("This is printed using {} macro.\n", "print!");
    println!("This is printed using {} macro. New line auto added.", "println!");

    serial_print!("This is printed using {} macro\n", "serial_print");
    serial_println!("This is printed using {} macro", "serial_println");

    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize(); }  // unsafe as misconfigured PIC can cause undefined behavior
    x86_64::instructions::interrupts::enable();  // executes `sti` instruction - set interrupts

    x86_64::instructions::int3();
    println!("It did not crash on breakpoint!");

    // this will cause kernel stack overflow, which will throw page
    // fault, since we don't have a handler for that, a double page
    // fault will be thrown; if IST is not implemented, then triple
    // fault be thrown i.e. system will reboot
    // fn a() {
    //     a();
    // }
    // a();

    // this can cause deadlock as PIT interrupt handler also have
    // print statement and interrupt happens while print macro is executing
    // loop {
    //     print!("-");
    //     for _ in 1..10000 {}
    // }

    let (l4_page_table, _) = x86_64::registers::control::Cr3::read();
    println!("Level 4 page table address: {:?}", l4_page_table.start_address());
    // this causes page fault due to write on invalid address
    // let ptr = 0xdeadbeef as *mut u32;
    // unsafe { *ptr = 42; }

    println!("It did not crash!");
    hlt_loop();
}
