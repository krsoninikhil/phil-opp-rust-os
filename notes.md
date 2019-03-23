## OS

- When computer is turned on, first thing that runs is firmware code
  (BIOS/UEFI) stored in ROM. It performs [Power-on self test][0],
  detects RAM and pre-initializes the CPU and hardware.
- If bootable disk is attached, then, control is transfered to the
  first 512 bytes of executable code stored in disk, which is called
  bootloader or first stage of bootloader which will load other
  stages.
- Bootloader will load the kernel image into memory, switch in-between
  required CPU modes and pass certain information from BIOS to kernel.
- FSF spec for bootloader is [Multiboot][1] and reference
  implementation is Grub. That means, Grub can load any Multiboot
  compliant OS.
- **Single Instruction Multi-Data (SIMD):** A set of standards (MMX,
  SSE, AVX for x86_64) for using registers in way to inficiently
  perform same option on multiple data points. Think of it like using
  vectorized operations instead of using loop for same
  operation. While this can significantly speed up programs, it's not
  good for kernel to use this for itself, because kernel needs to back
  up all the registers on earch interrupt which would be slow since
  SIMD registers are relatively large. Floating point uses SSE
  registers, so if SIMD is disabled and floating point had to be used,
  LLVM can do this by using `soft-float` which used software functions
  to emulate same operation on normal integers.
- **VGA Buffers:** Special memory area mapped to VGA hardware which
  contains the content displayed on the screen.
- **Spinlock:** Basic mutex that instead of blocking, threads tries
  locking again and again untill mutex if is free again.
- **Serial Port**: Legacy communication port to system, preceding
  USB. OS simulator can redirect the bytes sent over serial port to
  host OS. ICs that implement serial interface are called UART
  chips. These chips uses port mapped I/O.
- Peripheral devices and CPU can communicate either via memory mapped
  I/O like `0xb8000` for VGA buffer or port mapped I/O which uses
  different instructions (`in`, `out`) and address space than simple
  memory access.
- *CPU Exceptions:* When something illegal happens like devide by 0 or
  accessing illegal memory addresses, CPU throws [around 20 types][3]
  of exeption e.g. Page fault, double fault, triple faults etc.
- Handler functions for these exceptions are listed in table called
  IDT (Interrupt Descriptor Table), in a 16 bytes predefined format at
  predefined index.
- *Calling conventions* specify the details of function calls like
  where function parameters are place or how results are
  returned. They also defines *preserved* and *scratch* registers. `C`
  uses conventions specified in System V ABI.
- *Preserved Registers* are backed up by the called funtion
  i.e. _callee-saved_.
- *Scratch Registers* are backed up by the caller function before
  calling another function i.e. _caller-saved_.
- Since exception handler might run in different context, an specific
  convetion is used.
- Exception handlers uses x86 interrupt calling conventions which
  restores all registers values on function return to their original
  value.
- Exceptions pass exception stackframe to handler function. Some also
  pass a error code with stackframe.
- *Breakpoint Exception:* Defined at index 3 in IDT, it occurs when
  `int3` instruction is executed on CPU. Debugger replaces the current
  instruction with `int3` when breakpoint needs to be set.
- *Double fault* exception is thrown when their is error in calling
  original exception handler i.e. a particular exception is thrown
  after a specific exception e.g. a _page fault_ after _page fault_
  will cause _double fault_.
- If double fault is not handled too, fatal *triple fault* is thrown,
  which can't be caught and most hardware react with system reset.
- To prevent _triple fault_, _double fault_ needs to be handled
  correctly. Stack has to be valid (not on gaurd page) when _double
  fault_ handler is invoked as it also requires stack to place stack
  frame.
- *Guard Page:* Special memory page at the bottom of stack to detect
  stack overflow. This page is not mapped to any physical memory so
  accessing it causes _page fault_.
- *Interrupt Stack Table (IST):* List of 7 pointer to known good
  stacks to which hardware can switch before calling handler
  function. This can avoid the _triple fault_ in case, kernel stack
  overflows and _double fault_ handler arguments (exception
  stackframe) cannot be pushed to stack which will cause _triple
  fault_ if stack is not switched. `options` field in IDT handler
  entry specifies if and to which stack hardware should switch to.
- *Task State Segment (TSS):* Data structure which holds 2 stack
  tables - IST and Privilege Stack Table. Later is used to switch
  stack when privilege level changes. Linux x86_64 only uses stack
  table pointers and I/O port permission bitmap features of TSS.
- TSS uses segmentation system so we need to add an entry to GDT.
- *Global Descriptor Table:* Structure that contains segment of a
  program. It was used for memory segmentation and to provide virtual
  addresses before Paging was a thing. Still used for few things like
  loading TSS and configuring user/kernel mode.
- *Segment Selector:* An offset in GDT to specify which descriptor to
  use, like `index * element size` in an array.
- *Segment Selector Registers:* Used by processor to get different
  segment selector values e.g. `CS`, `DS`, etc. Needs to be updated
  once the GDT is loaded.
- *Hardware Interrupts:* Async notification to CPU from attached
  hardware devices.
- *Interrupts Controller* are separately attached to CPU which
  aggregates intrerrupts from all devices and notifies to CPU.
- *Intel 8259 PIC* (Programmable Interrupts Controller) was used
  before *APIC* but its interface still supported and is easier to
  implement. Typically 2 of these were chained together with fixed
  mapping to it's communication lines from 0-15.
- Each PIC can be configured by 2 I/O ports - command and data.
- CPU start listening to interrupts on executing `sti` instruction.
- By default PIT (Programmable Interval Timer) interrupts are enabled
  which needs to be handled if interrupts are enabled on CPU or a
  double fault will occur in absence of handler.
- PIC expect an explict 'end of interrupt' signal before it can send
  next interrupt. EOI signal tells PIC that interrupt has been
  processed. So handler function also needs to sent EOI signal.
- *Deadlock* occurs when a thread try to aquire a lock that will never
  become free.
- Keyboard interrupts are also enabled by default and next interrupt
  is blocked untill scancode of pressed key is read from keyboard's
  data port.

## Rust

- For a rust binary with standard library, execution is like: C
  runtime (`crt0`) -> rust runtime (`start` is entrypoint) -> program
  `main` function
- Linker options can be passed using `-C link-arg` option of `rustc`
  while compiling

  ```bash
  > cargo rustc -- -C link-arg=-nostartfiles
  ```

- Rust nightly can be used by adding `rust-toolchain` file in root
  saying `nightly`.
- While compiling Rust programs, target machine can be configured
  using pre-defined target triples or custom defined, using a json
  file specifying all the required options.
- For `no_std` crates, `core` and `compiler_builtins` libraries are
  implicitly linked which provides Rust basic types and lower lever
  functions expected by LLVM. These libs come precompiled with Rust
  compiler and are valid for pre-defined target triples.
- `cargo-xbuild` is a wrapper around `cargo` which can be used to
  compile sysroot crates (core, compiler_builtins and alloc) for
  custom targets.
- `unsafe` block of code or function can do operations which are not
  allowed by compiler like derefencing raw pointers and accessing or
  modifying mutable static variable.
- No implicity type casting in Rust. Can be done explicitly using
  `as`.
- `trait`s are almost like `interface`s in Golang or Java except
  `trait` can have default implementation. `impl` keyword is used to
  implement a `trait` `for` a `struct`. `impl` can also implement a
  `struct` without specifying a `trait`, just like Golang.
- Default implementation of some traits can be used as a implentation
  for a `struct` by using `derive` attribute. It's like interitence.
  e.g. `#[derive(Copy)]`.
- Compiler might optimize and omit the memory writes that are not
  accessed.
- Specifying memory writes as `volatile` tells compiler that value
  might change from somewhere else and should not be optimized.
- As of now, raw pointers can not be referenced in static objects.
  `lazy_static` crate can does this by computing pointer value only at
  runtime.
- `#[macro_export]` attribute brings macro to crate root and make it
  available to other modules and crates.
- Macro variable `$crate` expands in a way so macros can be directly
  used in same or external crate without breaking underlying function
  usage path.
- `cfg_attr` can be used to conditionally set the attributes.
- `Box::new()` allocates heap memory pointer and `Box::leak()` takes
  it's ref and returns mutable reference.
- Element type of array needs to derive `Copy` trait if wish to do
  initialization that copies the single value to all indices.
- Cargo features table can be used to add conditions for conditionaly
  with `cfg` attribute.
- `cargo test` builds all crates including independent binaries.
- Calling convention can be specified for a function e.g.
  `extern "C" fn`.
- `x86_64` crate provides IDT and `ExceptionStackFrame` implementation.
- To avoid null related issues, Rust have `Option<T>` enum, which can
  be `None` or `Some` with value of type `T`. Value from `Some` can be
  extracted by pattern matching.
- `if let` is short syntax for one arm pattern matching and simplify
  getting value from `Some`.

## Implementation

### Post 1 (A Freestanding Rust Binary)

- Compiled binary cannot link `std` lib, as it used OS specific
  features, which we are building here.
- To compile without `std`, we need to define what happens on panic
  and disable stack unwinding as well as default entry point.
- Provide our own entry point by overriding default entry point function
  (`_start` for linux, `main` for macOS), it must be available outside
  module and use C calling convention. Since linker looks for literal
  function name, disable name mangling.
- While compiling above implementation, proper options need to be
  passed to linker which are platform specific. On Linux, linker must
  not link startup routine of C runtime and on macOS, it must link
  `libSystem` as statically linked binary are not supported in macOS.

  ```bash
  # on Linux
  > cargo rustc -- -C link-arg=-nostartfiles
  # on macOS
  > cargo rustc -- -C link-arg=-lSystem
  ```

### Post 2 (A Minimal Rust Kernel)

- Binary built in last section can run on Linux or macOS target but we
  want to run it on bare metal. Switch to Rust nightly as some
  required features are experimental.
- Specify a custom target to compiler specifying options like no OS,
  no support for stack unwinding, which linker to use, [disable
  SIMD][2], enable `soft-float` as Rust core uses float, etc. We'll
  use DLL (LLVM) linker which in case of no targe OS, uses Linux
  convensions i.e. looks for `_start` for entrypoint.
- We'll also need to build all target specific linked libs that comes
  with Rust compiler and doesn't support custom target triples, like
  `core` and `compiler_builtins`. Then build our program using new
  libs. This can be done by:

  ```bash
  # this will provide `cargo xbuild` command
  > cargo install cargo-xbuild
  # we need source code of libs we are compiling
  > rustup component add rust-src
  # compile libs and use them to build program for provided target
  > cargo xbuild --target target-triple.json
  ```

- Now our binary has a bare metal target.
- Use VGA text buffer in entrypoint function to print text on screen.
- Allow `bootloader` crate to load our kernel by adding it as dependency.
- Create a bootable disk image by combining `bootloader` and
  kernel. `bootimage` tool can do this.

  ```bash
  # install bootimage
  > cargo install bootimage --version "^0.5.0"
  # combine bootloader with our program (kernel)
  > bootimage build --target target-triple.json
  ```

- Generated
  `target/target-triple/debug/bootimage-phil-opp-rust-os.bin` file can
  be written to a USB and can be booted on real machine.

### Post 3 (VGA Text Mode)

- To support text formating later on, refactor code for writing to VGA
  buffer in a separate module with a safe interface to write and hiding
  all unsafe operations.
- To make buffer writes `volatile`, add `volatile` crate as dependency
  and use that instead of directly writing.
- To use Rust built-in formatting macros (`write`, `writeln!`),
  implement `write_str` method of `core::fmt::Write` trait in our
  writer object.
- Adding newline in buffer can be done by shifting all values by 1 row
  and fill last row with whitespaces.
- To make our writer object available globally, add `lazy_static` as
  dependency specifying aobut `no_std` and make writer static using
  it.
- To make `writer` writable, add `spin` crate as dependency and use
  spinlock to make it mutable as mutable static are unsafe and
  discouraged.
- Implement `_print` function to write using `Writer` object and use
  that to implement `print!` macro and then `println!`.
- Now `println!` can be used directly in `_start`.

### Post 4 (Unit Testing)

- Add conditional config attributes to include `std` and not include
  `no_std` related code while executing tests as testing will be done
  on host machine.
- Create buffer object initialized with space character using
  `array-init` as `Volatile` doesn't have `Copy` trait.
- Now unit testing for `write_byte` and `write_string` can be done.

### Post 5 (Integration Tests)

- Use serial port for testing kernel's output on target machine (OS
  simulator) from host machine.
- Almost all UART models are compatible with 16550 UART, so use
  `uart_16550` crate for communicating with serial port. Add it as
  dependency.
- Implement a global safe interface with print macros vgfor writing to
  serial port just like we did for writing to VGA buffer.
- Unlike our VGA buffer writer, `uart_16550::SerialPort` writer
  already implements `Write` trait, so don't need to implement
  `write_str` ourselves. We use external crate in this case to avoid
  writing assembly which is required for writing to serial port.
- `-serial mon:stdio` option needs to be passed while starting QEMU to
  redirect written bytes to stdout of host.
- To be able to shutdown would require implementing complex APM or
  ACPI, so alternatively use QEMU's feature of adding a device
  `isa-debug-exit` at any unused port (`0xf4`) specifying port size.
- QEMU GUI can be hidden by padding `-display none` option while
  starting it.
- Full command to start QEMU becomes:
  ```bash
  qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none
  ```
  or with `bootimage`:
  ```bash
  bootimage run -- \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none
  ```
- Write to added I/O port (`0xf4`) using `x86_64` crate. Add it as
  dependency.
- Each integration test should run in isolation and using Cargo
  features to conditionaly compile would require creating too many
  features, so isolate each integration test in separate binary. To
  avoid duplicating code in each binary, separate out common code from
  main executable to a library.
- Write each test as executable binary in `src/bin/` with result from
  Post 1 as boilerplate. Start by testing if `_start` and `panic` are
  working as expected.
- To run integration test, we need to build tests, run in qemu and
  verify output from serial port. This can be done using bootimage by:
  ```bash
  bootimage test
  ```
  This will run all binaries named as `test-*.rs`.

### Post 6 (CPU Exceptions)

- Use `x86_64` crate to add exception handler function to IDT. Start
  with `breakpoint` and create a new module `interrupts` for handlers.
- Since exception can occur at any point, IDT needs to have
  `'static` lifetime and should be mutable, so use `lazy_static` to do
  load it.
- Write a integration test for testing `breakpoint` exception.

### Post 7 (Double Faults)

- Write a handler for double fault.
- Create a TSS global structure in a new module. Create a stack and set
  it to some (0) IST index in this TSS.
- Create GDT structure to load this TSS and update the segment
  selector registers.
- Point _double fault_ handler to this IST index.
- Write integration test for testing if GDT and TSS are loaded and
  stacking switching is working on stack overflow.

### Post 8 (Hardware Interrupts)

- Configure PICs to use vectors numbers that doesn't conflict with
  exceptions i.e. 32-47. `pic8259_simple` crate can be used to do so. Add it as
  dependency.
- Initialize PICs with configured vector number offset.
- Enable CPU interrupts.
- Add a timer interrupt handler as timer interrupts are on by default.
- Hanlder also need to notify end of interrupt to PIC.
- Currently, deadlock can occur if interrupt handler tries to print
  something when main thread have the writer lock as main thread will
  wait for interrupt handler to finish which is waiting for lock to be
  free.
- This deadlock can be provoked by calling `print` on loop in main
  function and having a `print` in handler too.
- To avoid this, one solution is to disable interrupts while aquiring
  a lock on writer mutex.
- Use `hlt` instruction instead of infinite loop while not doing
  anything to save hardware resoureces.
- As of now, pressing any keyboard key will cause double fault as no
  handler is present, so add a keyboard interrupt handler which also
  reads the scancode.
- Add scancode to actual key mapping using `pc-keyboard` crate.

## Additional Notes on Rust

- Variables are immutable by default.
- Variable that holds the reference to a memory allocated in heap is
  the owner of that memory. Memory is droped the moment variable goes
  out of scope.
- When a copy is made, ownership is moved to new variable and original
  variable becomes invalid i.e. cannot access memory anymore. This
  happens for types that doesn't implement `Copy` trait.
- To make a deepcopy i.e. copy the allocated heap memory also, `Clone`
  is implemented.
- When a reference is passed to new variable, it's called borrowing
  which is immutable by default and memory is not droped until owner
  goes out of scope. Borrower going out of scope does nothing to owner
  or memory.
- At a time in a scope, only one mutable reference or borrower can exist.
- If a allocated memory reference exist that must be owner
  i.e. compiler ensures that no dangling pointer exists.



[0]: https://en.wikipedia.org/wiki/Power-on_self-test
[1]: https://wiki.osdev.org/Multiboot
[2]: #os
[3]: http://wiki.osdev.org/Exceptions
