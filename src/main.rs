#![cfg_attr(not(test), no_std)]  // don't link std library as we won't have it
#![cfg_attr(not(test), no_main)]  // don't call main as we need to define our own entry point
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use bootloader::BootInfo;
use x86_64::structures::paging::{MapperAllSizes, Page};
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
// #[no_mangle]  // to preserve the function name, name mangling needs to disabled for this function
// pub extern "C" fn _start() -> ! {  // a better implementation of `_start` is provided by `bootloader` crate, so use that instead
bootloader::entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    print!("This is printed using {} macro.\n", "print!");
    println!("This is printed using {} macro. New line auto added.", "println!");

    serial_print!("This is printed using {} macro\n", "serial_print");
    serial_println!("This is printed using {} macro", "serial_println");

    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize(); }  // unsafe as misconfigured PIC can cause undefined behavior
    x86_64::instructions::interrupts::enable();  // executes `sti` instruction - set interrupts

    x86_64::instructions::interrupts::int3();
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

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };  // create memory mapper
    let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

    // map page with a random address to VGA buffer frame
    let page = Page::containing_address(x86_64::VirtAddr::new(0xdeadbeef));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // try writing to mapped page
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // try converting these virtual addresses into physical
    let addresses = [0xb8000, 0x20010a, boot_info.physical_memory_offset];
    for &address in &addresses {
        let virt = x86_64::VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    println!("It did not crash!");
    hlt_loop();
}
