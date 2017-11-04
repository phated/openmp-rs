# OpenMP linkage for Rust programs

NB: this crate won't enable OpenMP unless you also pass `-fopenmp` to the C compiler (e.g. `cc.flag("-fopenmp")` in cc-rs). This is solely for linking OpenMP-dependent C libraries with Rust code (in Rust use Rayon, etc.).

## Requirements

 * `libgomp.*` present in a directory printed by `cc -print-search-dirs`,
 * `-fopenmp` flag set for any C code linked with the Rust program.

It should work with recent versions of GCC.

## Usage

You can set `static` feature or set `OPENMP_STATIC` env var to link OpenMP statically, so that executables using it are usable on machines without a compiler installed.

### macOS

In macOS Clang pretends to be the `gcc` executable, but doesn't support OpenMP. Install `gcc-7` from Homebrew and compile with `CC=gcc-7 cargo build`. You may need to run `cargo clean` first if it doesn't take effect.

Static linking is recommended on macOS.
