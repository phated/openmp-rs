#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::*;
pub enum omp_nest_lock_t {}
pub enum omp_lock_t {}

pub const omp_sched_t_omp_sched_static: omp_sched_t = 1;
pub const omp_sched_t_omp_sched_dynamic: omp_sched_t = 2;
pub const omp_sched_t_omp_sched_guided: omp_sched_t = 3;
pub const omp_sched_t_omp_sched_auto: omp_sched_t = 4;
pub type omp_sched_t = c_uint;
pub const omp_proc_bind_t_omp_proc_bind_false: omp_proc_bind_t = 0;
pub const omp_proc_bind_t_omp_proc_bind_true: omp_proc_bind_t = 1;
pub const omp_proc_bind_t_omp_proc_bind_master: omp_proc_bind_t = 2;
pub const omp_proc_bind_t_omp_proc_bind_close: omp_proc_bind_t = 3;
pub const omp_proc_bind_t_omp_proc_bind_spread: omp_proc_bind_t = 4;
pub type omp_proc_bind_t = c_uint;
pub const omp_lock_hint_t_omp_lock_hint_none: omp_lock_hint_t = 0;
pub const omp_lock_hint_t_omp_lock_hint_uncontended: omp_lock_hint_t = 1;
pub const omp_lock_hint_t_omp_lock_hint_contended: omp_lock_hint_t = 2;
pub const omp_lock_hint_t_omp_lock_hint_nonspeculative: omp_lock_hint_t = 4;
pub const omp_lock_hint_t_omp_lock_hint_speculative: omp_lock_hint_t = 8;
pub type omp_lock_hint_t = c_uint;

extern "C" {
    pub fn omp_set_num_threads(arg1: c_int);
    pub fn omp_get_num_threads() -> c_int;
    pub fn omp_get_max_threads() -> c_int;
    pub fn omp_get_thread_num() -> c_int;
    pub fn omp_get_num_procs() -> c_int;
    pub fn omp_in_parallel() -> c_int;
    pub fn omp_set_dynamic(arg1: c_int);
    pub fn omp_get_dynamic() -> c_int;
    pub fn omp_set_nested(arg1: c_int);
    pub fn omp_get_nested() -> c_int;
    pub fn omp_init_lock(arg1: *mut omp_lock_t);
    pub fn omp_init_lock_with_hint(arg1: *mut omp_lock_t, arg2: omp_lock_hint_t);
    pub fn omp_destroy_lock(arg1: *mut omp_lock_t);
    pub fn omp_set_lock(arg1: *mut omp_lock_t);
    pub fn omp_unset_lock(arg1: *mut omp_lock_t);
    pub fn omp_test_lock(arg1: *mut omp_lock_t) -> c_int;
    pub fn omp_init_nest_lock(arg1: *mut omp_nest_lock_t);
    pub fn omp_init_nest_lock_with_hint(arg1: *mut omp_lock_t, arg2: omp_lock_hint_t);
    pub fn omp_destroy_nest_lock(arg1: *mut omp_nest_lock_t);
    pub fn omp_set_nest_lock(arg1: *mut omp_nest_lock_t);
    pub fn omp_unset_nest_lock(arg1: *mut omp_nest_lock_t);
    pub fn omp_test_nest_lock(arg1: *mut omp_nest_lock_t) -> c_int;
    pub fn omp_get_wtime() -> f64;
    pub fn omp_get_wtick() -> f64;
    pub fn omp_set_schedule(arg1: omp_sched_t, arg2: c_int);
    pub fn omp_get_schedule(arg1: *mut omp_sched_t, arg2: *mut c_int);
    pub fn omp_get_thread_limit() -> c_int;
    pub fn omp_set_max_active_levels(arg1: c_int);
    pub fn omp_get_max_active_levels() -> c_int;
    pub fn omp_get_level() -> c_int;
    pub fn omp_get_ancestor_thread_num(arg1: c_int) -> c_int;
    pub fn omp_get_team_size(arg1: c_int) -> c_int;
    pub fn omp_get_active_level() -> c_int;
    pub fn omp_in_final() -> c_int;
    pub fn omp_get_cancellation() -> c_int;
    pub fn omp_get_proc_bind() -> omp_proc_bind_t;
    pub fn omp_get_num_places() -> c_int;
    pub fn omp_get_place_num_procs(arg1: c_int) -> c_int;
    pub fn omp_get_place_proc_ids(arg1: c_int, arg2: *mut c_int);
    pub fn omp_get_place_num() -> c_int;
    pub fn omp_get_partition_num_places() -> c_int;
    pub fn omp_get_partition_place_nums(arg1: *mut c_int);
    pub fn omp_set_default_device(arg1: c_int);
    pub fn omp_get_default_device() -> c_int;
    pub fn omp_get_num_devices() -> c_int;
    pub fn omp_get_num_teams() -> c_int;
    pub fn omp_get_team_num() -> c_int;
    pub fn omp_is_initial_device() -> c_int;
    pub fn omp_get_initial_device() -> c_int;
    pub fn omp_get_max_task_priority() -> c_int;
    pub fn omp_target_alloc(
        arg1: c_ulong,
        arg2: c_int,
    ) -> *mut c_void;
    pub fn omp_target_free(arg1: *mut c_void, arg2: c_int);
    pub fn omp_target_is_present(
        arg1: *mut c_void,
        arg2: c_int,
    ) -> c_int;
    pub fn omp_target_memcpy(
        arg1: *mut c_void,
        arg2: *mut c_void,
        arg3: c_ulong,
        arg4: c_ulong,
        arg5: c_ulong,
        arg6: c_int,
        arg7: c_int,
    ) -> c_int;
    pub fn omp_target_memcpy_rect(
        arg1: *mut c_void,
        arg2: *mut c_void,
        arg3: c_ulong,
        arg4: c_int,
        arg5: *const c_ulong,
        arg6: *const c_ulong,
        arg7: *const c_ulong,
        arg8: *const c_ulong,
        arg9: *const c_ulong,
        arg10: c_int,
        arg11: c_int,
    ) -> c_int;
    pub fn omp_target_associate_ptr(
        arg1: *mut c_void,
        arg2: *mut c_void,
        arg3: c_ulong,
        arg4: c_ulong,
        arg5: c_int,
    ) -> c_int;
    pub fn omp_target_disassociate_ptr(
        arg1: *mut c_void,
        arg2: c_int,
    ) -> c_int;
}
