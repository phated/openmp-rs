# OpenMP library linkage for [Rust](https://www.rust-lang.org) programs

[This crate](https://crates.rs/crates/openmp-sys) allows using OpenMP-dependent C code with Rust. It makes Cargo link to OpenMP, so that C static libraries linked with Rust programs can use OpenMP.

It can't be used with pure Rust programs ([Rayon](https://lib.rs/crates/rayon) is a better choice for Rust).

NB: This crate can't automatically enable OpenMP for C code compiled from build scripts. You also need to pass the appropriate OpenMP-enabling flag to the C compiler (see usage below). It may be necessary to perform `cargo clean` and rebuild if settings don't take effect.

## Requirements

 * Rust 1.42 or later
 * OpenMP libraries and headers
    - in a directory printed by `cc -print-search-dirs`, or
    - provided by the `libomp` [formula](https://formulae.brew.sh/formula/libomp) or [port](https://ports.macports.org/port/libomp) installed to standard prefix on macOS, or
    - `vcomp.dll` et al. with MSVC on Windows, or
    - at locations specified in `LIBRARY_PATH` and `CFLAGS` respectively at compile time:
      ```sh
      LIBRARY_PATH="<path containing libomp.{so|dylib|lib|a}>:<other library paths>"
      CFLAGS="-I<path containing omp.h> <other C flags>"
      ```
 * OpenMP-enabling flag set for any C code linked with the Rust program

## Usage

### 1. Adding Rust dependency

Add `openmp-sys` as a runtime dependency (e.g. `cargo install cargo-edit; cargo add openmp-sys`) and then add to your `lib.rs`:

```rust
extern crate openmp_sys;
```

### 2. Configuring C compiler

The C code being linked *must* be compiled with an OpenMP-enabling flag. For projects that use the [`cc` crate](https://lib.rs/cc) it can be made easier. If you add `openmp-sys` also as a dev-depenency, it will set `DEP_OPENMP_FLAG` [env var](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts) for [your `build.rs` script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) with an appropriate flag (or sometimes, flags) such as `-fopenmp` or `/openmp`, depending on the target compiler. Pass this to [`cc` build](https://docs.rs/cc) this way:

```rust
let mut cc_build = cc::Build::new();
env::var("DEP_OPENMP_FLAG").unwrap().split(" ").for_each(|f|{cc_build.flag(f);});
```

### Static linking

Optionally, you can enable `static` feature or set `OPENMP_STATIC=1` env var to link OpenMP statically, so that executables using it are usable on machines without a compiler installed.

```toml
[dependencies.openmp]
features = ["static"]
version = "0.1"
```

### Custom CC

It's possible to specify another C compiler at build time with the `CC` environment variable. However, Cargo will still default to `cc` while linking, regardless of what is chosen while compiling. If issues emerge in this regard, set [`CARGO_TARGET_<triple>_LINKER`](https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinker) or the respective item in `~/.cargo/config` to override this.

### macOS

On macOS, both Apple Clang and LLVM.org (upstream) Clang are supported, provided that a copy of `libomp` is available (as mentioned in the requirements above). This is not needed when using GCC, as it comes bundled with `libgomp`. If your program requires GCC, it is sufficient to do `CC=<gcc command> cargo build`.

Static linking is recommended on macOS.
