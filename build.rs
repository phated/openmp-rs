extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    let wants_static = cfg!(feature = "static") || env::var_os("OPENMP_STATIC").is_some();

    let mut cc = cc::Build::new();
    cc.flag("-print-search-dirs");
    let comp = cc.get_compiler();

    if comp.is_like_msvc() {
        println!("cargo:flag=/openmp");
        println!("cargo:rustc-link-lib=vcomp");
        if wants_static {
            println!("cargo:warning=Visual Studio doesn't support static OpenMP");
        }
        return;
    }
    if comp.is_like_clang() && !comp.is_like_gnu() {
        println!("cargo:warning=Clang may not support OpenMP. Try using GCC instead (`export CC=<real-gcc-exe>`)");
    }
    println!("cargo:flag=-fopenmp");

    let mut compiler_libs = Vec::new();
    let out = String::from_utf8(comp.to_command().output().expect("running compiler to get the lib paths").stdout).unwrap();
    for line in out.split('\n').filter(|l| l.starts_with("libraries: =")) {
        let line = line.trim_left_matches("libraries: =");
        compiler_libs.extend(env::split_paths(line));
    }

    if wants_static {
        if comp.is_like_gnu() && !comp.is_like_clang() {
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
