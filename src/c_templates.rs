use ::core::ffi::{c_int, c_char};
use crate::types::{
    c_types::{c_uid_t, c_gid_t},
    c_structs::{Cdev, Ucred, Mtx}
};
use crate::cdev::Cdevsw;
use crate::mutex::Mutex;





unsafe extern "C" {
    pub fn uprintf(fmt: *const c_char, ...) -> c_int;


    /*
     * Device interaction
     */
    pub fn make_dev_p(
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

    pub fn destroy_dev(dev: *mut Cdev);


    /*
     * Mutex
     */
    pub fn mtx_init(mutex: *mut Mtx, name: *const c_char, mtype: *const c_char, opts: c_int);
    pub fn mtx_destroy(mutex: *mut Mtx);
    pub fn mtx_lock(mutex: *mut Mtx);
    // pub fn mtx_try_lock();
    pub fn mtx_unlock(mutex: *mut Mtx);
    pub fn mtx_initialized(mutex: *const Mtx) -> c_int;
}
