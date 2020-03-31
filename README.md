# OpenMP library linkage for [Rust](https://www.rust-lang.org) programs

[This crate](https://crates.rs/crates/openmp-sys) allows using OpenMP-dependent C code with Rust. It makes Cargo link to OpenMP, so that C static libraries linked with Rust programs can use OpenMP.

It can't be used with pure Rust programs ([Rayon](https://lib.rs/crates/rayon) is a better choice for Rust).

NB: this crate can't automatically enable OpenMP for C code compiled from build scripts. You also need to pass `-fopenmp`/`/openmp` flag to the C compiler (see usage below).

## Requirements

 * Rust 1.42 or later
 * `libgomp.*` present in a directory printed by `cc -print-search-dirs`, or `vcomp.dll` on Windows
 * OpenMP-enabling flag set for any C code linked with the Rust program.

## Usage

### 1. Adding Rust dependency

Add `openmp-sys` as a runtime dependency (e.g. `cargo install cargo-edit; cargo add openmp-sys`) and then add to your `lib.rs`:

```rust
extern crate openmp_sys;
```

### 2. Configuring C compiler

The C code being linked *must* be compiled with an OpenMP-enabling flag. For projects that use the [`cc` crate](https://lib.rs/cc) it can be made easier. If you add `openmp-sys` also as a dev-depenency, it will set `DEP_OPENMP_FLAG` [env var](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts) for [your `build.rs` script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) with an appropriate flag such as `-fopenmp` or `/openmp`, depending on the target compiler. Pass this to [`cc` build](https://docs.rs/cc) this way:

```rust
cc_build.flag(&std::env::var("DEP_OPENMP_FLAG").unwrap());
```

### Static linking

Optionally, you can enable `static` feature or set `OPENMP_STATIC=1` env var to link OpenMP statically, so that executables using it are usable on machines without a compiler installed. Only GCC supports this.

```toml
[dependencies.openmp]
features = ["static"]
version = "0.1"
```

### macOS

In macOS Clang pretends to be the `gcc` executable, but doesn't support OpenMP. Install `brew install gcc` from Homebrew and compile with `CC=gcc-9 cargo build`. You may need to run `cargo clean` first if it doesn't take effect.

Static linking is recommended on macOS.
