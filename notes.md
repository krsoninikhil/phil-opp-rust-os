## OS


## Rust

- For a rust binary with standard library, execution is like:
  C runtime (`crt0`) -> rust runtime (`start` is entrypoint) -> program `main` function
- Linker options can be passed using `-C link-arg` option of `rustc`
  while compiling

  ```bash
  > cargo rustc -- -C link-arg=-nostartfiles
  ```


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
