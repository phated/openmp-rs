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
    let mut cmd = comp.to_command();
    let out = match cmd.output() {
        Ok(out) => String::from_utf8(out.stdout).unwrap(),
        Err(err) => {
            println!("cargo:warning=Error when setting up OpenMP via openmp-sys crate. Your C compiler doesn't seem to work. The command:\n\
                cargo:warning={:?}\n\
                cargo:warning=Failed because: {}\n\
                cargo:warning=(the PATH is: {:?}; CC is {:?})", cmd, err, env::var("PATH"), env::var("CC"));
            "".to_string()
        }
    };

    let libomp = PathBuf::from("/usr/local/opt/libomp/lib");
    if libomp.exists() {
        compiler_libs.push(libomp);
    }

    for line in out.split('\n').filter(|l| l.starts_with("libraries: =")) {
        let line = line.trim_start_matches("libraries: =");
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
    } else {
        println!("cargo:flag=-fopenmp");
    }

    let lib_names = if is_clang {&["omp", "iomp", "gomp"][..]} else {&["gomp"]};

    if wants_static {
        if comp.is_like_gnu() && !is_clang {
            find_and_link(&["gcc_eh"], true, &compiler_libs);
        }
        find_and_link(lib_names, true, &compiler_libs);
    } else {
        find_and_link(lib_names, false, &compiler_libs);
    };
}

fn find_and_link(lib_names: &[&str], statik: bool, in_paths: &[PathBuf]) {
    let names = lib_names.iter().copied().map(|lib_name| if statik {
        (lib_name, format!("lib{}.a", lib_name))
    } else {
        (lib_name, format!("{}{}{}", env::consts::DLL_PREFIX, lib_name, env::consts::DLL_SUFFIX))
    }).collect::<Vec<_>>();

    for path in in_paths {
        for (name, file) in &names {
            if path.join(file).exists() {
                println!("cargo:rustc-link-search=native={}", path.display());
                println!("cargo:rustc-link-lib{}={}", if statik {"=static"} else {""}, name);
                return;
            }
        }
    }
    let cc = env::var("CC").unwrap_or_else(|_| "cc".to_owned());
    println!("cargo:warning=openmp-sys is unable to find library {} for {} in {:?}", names[0].1, cc, in_paths);
}
