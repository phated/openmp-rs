# OpenMP library linkage for [Rust](https://www.rust-lang.org) programs

[This crate](https://crates.rs/crates/openmp-sys) allows using OpenMP-dependent C code with Rust. It makes Cargo link to OpenMP, so that C static libraries linked with Rust programs can use OpenMP.

It can't be used with pure Rust programs ([Rayon](https://crates.rs/crates/rayon) is a better choice for Rust).

NB: this crate can't automatically enable OpenMP for C code compiled from build scripts. You also need to pass `-fopenmp`/`/openmp` flag to the C compiler (see usage below).

## Requirements

 * `libgomp.*` present in a directory printed by `cc -print-search-dirs`, `vcomp.dll` on Windows
 * OpenMP-enabling flag set for any C code linked with the Rust program.

## Usage

### Linking

You can set `static` feature or set `OPENMP_STATIC=1` env var to link OpenMP statically, so that executables using it are usable on machines without a compiler installed.

```toml
[dependencies.openmp]
features = ["static"]
version = "0.1"
```

### Enabling

Cargo build scripts will get `DEP_OPENMP_FLAG` env var set which contains either `-fopenmp` or `/openmp`, depending on the target compiler. If you're building C code with cc-rs:

```rust
cc.flag(&env::var("DEP_OPENMP_FLAG").unwrap());
```

### macOS

In macOS Clang pretends to be the `gcc` executable, but doesn't support OpenMP. Install `brew install gcc` from Homebrew and compile with `CC=gcc-8 cargo build`. You may need to run `cargo clean` first if it doesn't take effect.

Static linking is recommended on macOS.
