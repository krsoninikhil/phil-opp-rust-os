#![cfg_attr(not(test), no_std)]  // don't link std library as we won't have it
#![feature(abi_x86_interrupt)]

pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod memory;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    // attach `isa-debug-exit` device on 0xf4 port while starting QEMU
    let mut port = Port::<u8>::new(0xf4);
    port.write(0);  // exit status will be `(passed value << 1) | 1` i.e. `1`
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
