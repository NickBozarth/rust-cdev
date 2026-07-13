use ::core::ffi::{c_int, c_char};
use crate::types::{
    c_structs::{Cdev, MakeDevArgs, Mtx, Ucred}, c_types::{c_gid_t, c_size_t, c_uid_t}
};
use crate::cdev::Cdevsw;


/*
 * NOTE I think if a function symbol is not found, everything breaks in a really weird way
 *  My experience had the rust_cdev_modevent symbol showing as undefined in the linker
 * NOTE Defining a symbol with the correct name and incorrect arguments leads to
 *  undefined behavior ( In many cases it crashes the os :( )
 *
 * IT IS RECCOMENDED TO ENCAPSULATE FUNCTIONS IN SAFE ABSTRACTIONS
 *  ex: Mutex/MutexGuard objects with safe calling of these functions and RAII
 */
unsafe extern "C" {
    /*
     * Userspace interaction
     */
    pub fn uprintf(fmt: *const c_char, ...) -> c_int;


    /*
     * Device interaction
     */
    pub(crate) fn make_dev_s(
        _args: *mut MakeDevArgs,
        _cdev: *mut *mut Cdev,
        _fmt: *const c_char,
        ...
    ) -> c_int;

    pub(crate) fn destroy_dev(dev: *mut Cdev);

    pub(crate) fn make_dev_args_init_impl(args: *mut MakeDevArgs, sz: c_size_t);


    /*
     * Mutex
     */
    pub(crate) fn mtx_init(mutex: *mut Mtx, name: *const c_char, mtype: *const c_char, opts: c_int);
    pub(crate) fn mtx_destroy(mutex: *mut Mtx);
    pub(crate) fn mtx_lock(mutex: *mut Mtx);
    // pub(crate) fn mtx_try_lock();
    pub(crate) fn mtx_unlock(mutex: *mut Mtx);
    pub(crate) fn mtx_initialized(mutex: *const Mtx) -> c_int;
}



#[macro_export]
macro_rules! uprintf {
    ($fmt: expr) => {
        unsafe { $crate::c_templates::uprintf($fmt) }
    };

    ($fmt:expr, $($args:expr)*) => {
        unsafe { $crate::c_templates::uprintf($fmt, ($($args),*)) }
    };
}
