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

## Rust

- For a rust binary with standard library, execution is like:
  C runtime (`crt0`) -> rust runtime (`start` is entrypoint) -> program `main` function
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

[0]: https://en.wikipedia.org/wiki/Power-on_self-test
[1]: https://wiki.osdev.org/Multiboot
[2]: #os
