use ::core::ffi::{c_int, c_char};
use crate::types::{
    c_structs::{Cdev, MakeDevArgs, Mtx, Ucred}, c_types::{c_gid_t, c_uid_t}
};
use crate::cdev::Cdevsw;



unsafe extern "C" {
    /*
     * Userspace interaction
     */
    pub fn uprintf(fmt: *const c_char, ...) -> c_int;


    /*
     * Device interaction
     */
    pub(crate) fn make_dev_p(
        _flags: c_int,
        _cdev: *mut *mut Cdev,
        _devsw: *mut Cdevsw,
        _cr: *mut Ucred,
        _uid: c_uid_t,
        _gid: c_gid_t,
        _mode: c_int,
        _fmt: *const c_char,
        ...
    ) -> c_int;

    pub(crate) fn make_dev_s(
        _args: *mut MakeDevArgs,
        _cdev: *mut *mut Cdev,
        _fmt: *const c_char,
        ...
    ) -> c_int;

    pub(crate) fn destroy_dev(dev: *mut Cdev);


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
