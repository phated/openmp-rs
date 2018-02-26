extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    let wants_static = cfg!(feature = "static") || env::var_os("OPENMP_STATIC").is_some();
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=OPENMP_STATIC");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let mut cc = cc::Build::new();
    cc.flag("-print-search-dirs");
    let comp = cc.get_compiler();

    let mut compiler_libs = Vec::new();
    let out = String::from_utf8(comp.to_command().output().expect("running compiler to get the lib paths").stdout).unwrap();
    for line in out.split('\n').filter(|l| l.starts_with("libraries: =")) {
        let line = line.trim_left_matches("libraries: =");
        compiler_libs.extend(env::split_paths(line));
    }

    // cc-rs often can't really tell them apart
    let is_clang = if comp.is_like_gnu() {
        compiler_libs.iter().filter_map(|p| p.to_str()).any(|path| path.contains("/clang/"))
    } else {
        comp.is_like_clang()
    };

    if comp.is_like_msvc() {
        println!("cargo:flag=/openmp");
        println!("cargo:rustc-link-lib=vcomp");
        if wants_static {
            println!("cargo:warning=Visual Studio doesn't support static OpenMP");
        }
        return;
    }
    if is_clang {
        println!("cargo:warning=Clang may not support OpenMP. Try using GCC instead (`export CC=<real-gcc-exe>`)");
    }
    println!("cargo:flag=-fopenmp");

    if wants_static {
        if comp.is_like_gnu() && !is_clang {
            search_path_for("libgcc_eh.a", &compiler_libs);
            println!("cargo:rustc-link-lib=static=gcc_eh");
        }
        search_path_for("libgomp.a", &compiler_libs);
        println!("cargo:rustc-link-lib=static=gomp");
    } else {
        search_path_for(&format!("{}gomp{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX), &compiler_libs);
        println!("cargo:rustc-link-lib=gomp");
    };

}

fn search_path_for(name: &str, in_paths: &[PathBuf]) {
    for path in in_paths {
        if path.join(name).exists() {
            println!("cargo:rustc-link-search=native={}", path.display());
            return;
        }
    }
    println!("cargo:warning=Unable to find library {} for {} in {:?}", name, env::var("CC").unwrap_or("cc".to_owned()), in_paths);
}
