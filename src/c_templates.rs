use ::core::ffi::{c_int, c_char};
use crate::types::{
    c_types::{c_uid_t, c_gid_t},
    c_structs::{Cdev, Ucred}
};
use crate::cdev::Cdevsw;





unsafe extern "C" {
    pub fn uprintf(fmt: *const c_char, ...) -> c_int;

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
}
