use std::env;
use std::path::{Path, PathBuf};

struct Womp {
    flag: &'static str,
    link: Vec<String>,
}

struct Compiler {
    is_msvc: bool,
    is_clang: bool,
    is_gcc: bool,
    is_apple_clang: bool,
    search_paths: Vec<PathBuf>,
}

fn main() {
    let wants_static = cfg!(feature = "static") || env::var_os("OPENMP_STATIC").is_some();
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=OPENMP_STATIC");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let comp = probe_compiler();
    let womp = find_openmp(wants_static, comp);

    println!("cargo:flag={}", womp.flag);
    if let Ok(s) = env::join_paths(&womp.link) {
        if let Some(s) = s.to_str() {
            println!("cargo:cargo_link_instructions={}", s);
        }
    }
    for l in &womp.link {
        println!("cargo:{}", l);
    }
}

fn probe_compiler() -> Compiler {
    let mut compiler_libs = Vec::new();
    env::set_var("LANG", "C");
    let comp = cc::Build::new()
        .flag("-v")
        .flag("-print-search-dirs")
        .get_compiler();
    let mut cmd = comp.to_command();
    let (out, err) = match cmd.output() {
        Ok(out) => (
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap(),
        ),
        Err(err) => {
            println!("cargo:warning=Error when setting up OpenMP via openmp-sys crate. Your C compiler doesn't seem to work. The command:\n\
                cargo:warning={:?}\n\
                cargo:warning=Failed because: {}\n\
                cargo:warning=(the PATH is: {:?}; CC is {:?})", cmd, err, env::var("PATH"), env::var("CC"));
            ("".to_string(), "".to_string())
        }
    };

    if let Ok(library_path) = env::var("LIBRARY_PATH") {
        for lib_dir in env::split_paths(&library_path) {
            compiler_libs.push(Path::new(&lib_dir).to_path_buf());
        }
    }

    for line in out
        .split('\n')
        .filter_map(|l| l.strip_prefix("libraries: ="))
    {
        compiler_libs.extend(env::split_paths(line));
    }

    // cc-rs often can't really tell them apart
    let is_clang = err.contains("clang") || comp.is_like_clang();
    let is_apple_clang = is_clang && err.starts_with("Apple");

    if is_clang && err.contains("apple-darwin") {
        if let Some(brew_prefix) = find_brew_prefix() {
            let lib_dir = Path::new(&brew_prefix);
            if lib_dir.exists() {
                compiler_libs.push(lib_dir.to_path_buf());
            }
        }
    }
    Compiler {
        is_msvc: comp.is_like_msvc(),
        is_gcc: comp.is_like_gnu() && !is_clang,
        is_clang,
        is_apple_clang,
        search_paths: compiler_libs,
    }
}

fn find_openmp(wants_static: bool, comp: Compiler) -> Womp {
    if comp.is_msvc {
        if wants_static {
            println!("cargo:warning=Visual Studio doesn't support static OpenMP. Ship vcomp.dll");
        }
        return Womp {
            flag: "/openmp",
            link: vec!["rustc-link-lib=vcomp".into()],
        };
    }

    let flag = if comp.is_apple_clang {
        "-Xpreprocessor -fopenmp"
    } else {
        "-fopenmp"
    };

    let mut out_libs = vec![];

    if wants_static && comp.is_gcc {
        find_and_link(&["gcc_eh"], true, &comp.search_paths, &mut out_libs);
        let target = std::env::var("TARGET").unwrap();
        if target == "aarch64-apple-darwin" {
            // gcc-11 on M1 uses ___aarch64_ldadd4_acq_rel due to -m outline-atomics used to build libgomp
            // so it needs it from libgcc.a (sadly, libatomic.a alone is not enough)
            find_and_link(&["gcc"], true, &comp.search_paths, &mut out_libs);
        }
    }

    let lib_names = if comp.is_clang {
        &["omp", "iomp", "gomp"][..]
    } else {
        &["gomp"]
    };
    find_and_link(lib_names, wants_static, &comp.search_paths, &mut out_libs);

    Womp {
        flag,
        link: out_libs,
    }
}

fn find_and_link(lib_names: &[&str], statik: bool, in_paths: &[PathBuf], out: &mut Vec<String>) {
    let names = lib_names
        .iter()
        .copied()
        .map(|lib_name| {
            if statik {
                (lib_name, format!("lib{}.a", lib_name))
            } else {
                (
                    lib_name,
                    format!(
                        "{}{}{}",
                        env::consts::DLL_PREFIX,
                        lib_name,
                        env::consts::DLL_SUFFIX
                    ),
                )
            }
        })
        .collect::<Vec<_>>();

    for path in in_paths {
        for (name, file) in &names {
            if path.join(file).exists() {
                out.push(format!("rustc-link-search=native={}", path.display()));
                out.push(format!(
                    "rustc-link-lib{}={}",
                    if statik { "=static" } else { "" },
                    name
                ));
                return;
            }
        }
    }
    let cc = env::var("CC").unwrap_or_else(|_| "cc".to_owned());
    println!(
        "cargo:warning=openmp-sys is unable to find library {} for {} in {:?}",
        names[0].1, cc, in_paths
    );
}

fn find_brew_prefix() -> Option<String> {
    let output = std::process::Command::new("brew")
        .arg("--prefix")
        .stdout(std::process::Stdio::piped())
        .output();

    match output {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(stdout) => Some(stdout.trim().to_string()),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
