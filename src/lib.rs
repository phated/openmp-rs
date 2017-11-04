/// NB: this crate won't enable OpenMP unless you also pass `-fopenmp`
/// to the C compiler (e.g. `cc.flag("-fopenmp")` in cc-rs).
///
/// This is solely for linking OpenMP-dependent C libraries with Rust code
/// and you probably shouldn't use any of the functions listed here.

pub mod ffi;

#[test]
fn it_works() {
    unsafe {
        assert!(ffi::omp_get_max_threads() > 0);
    }
}
