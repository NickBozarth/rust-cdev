#[allow(non_camel_case_types)]
pub mod c_types {
    use ::core::ffi::c_char;

    pub type c_size_t = usize;
    pub type c_uid_t = u32;
    pub type c_gid_t = u32;
    pub type c_caddr_t = *mut c_char;
    pub type c_vm_ooffset_t = u64;
    pub type c_vm_memattr_t = c_char;

    #[cfg(target_pointer_width = "64")]
    pub type c_vm_paddr_t = u64;
    #[cfg(target_pointer_width = "32")]
    pub type c_vm_paddr_t = u32;

    #[cfg(target_pointer_width = "64")]
    pub type c_vm_size_t = u64;
    #[cfg(target_pointer_width = "32")]
    pub type c_vm_size_t = u32;

    #[cfg(target_pointer_width = "64")]
    pub type c___uintptr_t = u64;
    #[cfg(target_pointer_width = "32")]
    pub type c___uintptr_t = u32;
}



pub mod c_structs {
    use ::core::ffi::{c_int, c_char, c_uint, c_void};
    use super::c_types::{c_size_t, c___uintptr_t};
    use crate::{cdev::Cdevsw, consts::{gid, uid}, types::c_types::{c_gid_t, c_uid_t}};

    #[repr(C)]
    pub struct MakeDevArgs {
        mda_size: c_size_t,
        pub mda_flags: c_int,
        pub mda_devsw: *mut Cdevsw,
        pub mda_cr: *mut Ucred,
        pub mda_uid: c_uid_t,
        pub mda_gid: c_gid_t,
        pub mda_mode: c_int,
        pub mda_uint: c_int,
        pub mda_si_drv1: *mut c_void,
        pub mda_si_drv2: *mut c_void,
    }


    impl MakeDevArgs {
        pub const fn default() -> Self {
            Self {
                mda_size:       size_of::<Self>(),
                mda_flags:      0,
                mda_devsw:      ::core::ptr::null_mut(),
                mda_cr:         ::core::ptr::null_mut(),
                mda_uid:        uid::ROOT,
                mda_gid:        gid::WHEEL,
                mda_mode:       0,
                mda_uint:       0,
                mda_si_drv1:    ::core::ptr::null_mut(),
                mda_si_drv2:    ::core::ptr::null_mut(),
            }
        }
    }


    /*
     * These are to be used internally for pub Mutex type and extern C functions
     */
    #[repr(C)]
    struct LockObject {
        pub lo_name:    *const c_char,
        pub lo_flags:   c_uint,
        pub lo_data:    c_uint,
        pub lo_witness: *mut Witness
    }
    impl LockObject {
        pub const fn new() -> Self {
            Self {
                lo_name:    ::core::ptr::null(),
                lo_flags:   0,
                lo_data:    0,
                lo_witness: ::core::ptr::null_mut(),
            }
        }
    }
    #[repr(C)]
    pub(crate) struct Mtx {
        lock_object:    LockObject,
        mtx_lock:       c___uintptr_t,
    }
    impl Mtx {
        pub const fn new() -> Self {
            Self {
                lock_object:    LockObject::new(),
                mtx_lock:       0x4,
            }
        }
    }


    /*
     * These should never be instantiated on by driver,
     *  they must be passed as an argument from an extern
     *  and should only be passed as a pointer
     */
    #[repr(C)]
    pub struct Cdev     { _private: [u8; 0] }
    #[repr(C)]
    pub struct Thread   { _private: [u8; 0] }
    #[repr(C)]
    pub struct Uio      { _private: [u8; 0] }
    #[repr(C)]
    pub struct Ucred    { _private: [u8; 0] }
    #[repr(C)]
    pub struct CFile    { _private: [u8; 0] }
    #[repr(C)]
    pub struct Bio      { _private: [u8; 0] }
    #[repr(C)]
    pub struct Knote    { _private: [u8; 0] }
    #[repr(C)]
    pub struct VmObject { _private: [u8; 0] }
    #[repr(C)]
    struct Witness      { _private: [u8; 0] }

    
}

pub mod d_functions {
    use ::core::ffi::{c_int, c_ulong};
    use super::{
        c_structs::{Cdev, Thread, Uio, Knote, CFile, VmObject, Bio},
        c_types::{c_caddr_t, c_vm_ooffset_t, c_vm_paddr_t, c_vm_memattr_t, c_vm_size_t}
    };

    pub type DOpenT         = unsafe extern "C" fn(dev: *mut Cdev, oflags: c_int, devtype: c_int, td: *mut Thread) -> c_int;
    pub type DFdopenT       = unsafe extern "C" fn(dev: *mut Cdev, oflags: c_int, td: *mut Thread, fp: *mut CFile) -> c_int;
    pub type DCloseT        = unsafe extern "C" fn(dev: *mut Cdev, fflag: c_int, devtype: c_int, td: *mut Thread) -> c_int;
    pub type DReadT         = unsafe extern "C" fn(dev: *mut Cdev, uio: *mut Uio, ioflag: c_int) -> c_int;
    pub type DWriteT        = unsafe extern "C" fn(dev: *mut Cdev, uio: *mut Uio, ioflag: c_int) -> c_int;
    pub type DIoctlT        = unsafe extern "C" fn(dev: *mut Cdev, cmd: c_ulong, data: c_caddr_t, fflag: c_int, td: *mut Thread) -> c_int;
    pub type DPollT         = unsafe extern "C" fn(dev: *mut Cdev, events: c_int, td: *mut Thread) -> c_int;
    pub type DMmapT         = unsafe extern "C" fn(dev: *mut Cdev, offset: c_vm_ooffset_t, paddr: *mut c_vm_paddr_t, nprot: c_int, memattr: *mut c_vm_memattr_t) -> c_int;
    pub type DStrategyT     = unsafe extern "C" fn(bp: *mut Bio) -> c_int;
    pub type DKqfilterT     = unsafe extern "C" fn(dev: *mut Cdev, kn: *mut Knote) -> c_int;
    pub type DPurgeT        = unsafe extern "C" fn(dev: *mut Cdev) -> c_int;
    pub type DMmapSingleT   = unsafe extern "C" fn(cdev: *mut Cdev, offset: *mut c_vm_ooffset_t, size: c_vm_size_t, object: *mut *mut VmObject, nprot: c_int) -> c_int;
}
