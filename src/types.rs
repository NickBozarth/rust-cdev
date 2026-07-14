




/*
 * The primary purpose of these is to match the exact signature of
 *  the original c functions
 */
#[allow(non_camel_case_types)]
pub mod c_types {
    use ::core::ffi::c_char;

    pub type c_size_t       = usize;
    pub type c_ssize_t      = isize;
    pub type c_uid_t        = u32;
    pub type c_gid_t        = u32;
    pub type c_caddr_t      = *mut c_char;
    pub type c_vm_ooffset_t = u64;
    pub type c_vm_memattr_t = c_char;
    pub type c_off_t        = u64;

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
    use crate::{c_templates::{make_dev_args_init_impl}, cdev::Cdevsw, types::c_types::{c_gid_t, c_off_t, c_ssize_t, c_uid_t}};

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
        pub fn default() -> Self {
            /*
             * All args are safe to be zeroed before initialization
             */
            let mut args: Self;
            unsafe { 
                args = ::core::mem::zeroed();
                make_dev_args_init_impl(&raw mut args, ::core::mem::size_of::<Self>());
            };

            args
        }
    }


    /*
     * These are to be used internally for pub Mutex type and extern C functions
     */
    /* <sys/_lock.h> */
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

    /* <sys/_mutex.h> */
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


    /* <sys/uio.h> */
    
    #[repr(C)]
    pub struct Uio {
        pub(crate) uio_iov:    *mut Iovec,
        pub(crate) uio_iovcnt: c_int,
        pub(crate) uio_offset: c_off_t,
        pub(crate) uio_resid:  c_ssize_t,
        pub(crate) uio_segflg: UioSeg,
        pub(crate) uio_rw:     UioRw,
        pub(crate) uio_td:     *mut Thread,
    }
    


    /*
     * These should never be instantiated on by driver,
     *  they must be passed as an argument from an extern
     *  and should only be passed as a pointer
     */
    #[repr(C)]
    pub struct Cdev         { _private: [u8; 0] }
    #[repr(C)]
    pub struct Thread       { _private: [u8; 0] }
    #[repr(C)]
    pub struct Ucred        { _private: [u8; 0] }
    #[repr(C)]
    pub struct CFile        { _private: [u8; 0] }
    #[repr(C)]
    pub struct Bio          { _private: [u8; 0] }
    #[repr(C)]
    pub struct Knote        { _private: [u8; 0] }
    #[repr(C)]
    pub struct VmObject     { _private: [u8; 0] }
    #[repr(C)]
    struct Witness          { _private: [u8; 0] }
    #[repr(C)]
    pub(crate) struct Iovec { _private: [u8; 0] }


    /* <sys/_uio.h> */
    #[allow(dead_code)]
    #[repr(C)]
    pub(crate) enum UioSeg {
        USERSPACE,
        SYSSPACE,
        NOCOPY
    }
    #[allow(dead_code)]
    #[repr(C)]
    pub(crate) enum UioRw {
        READ,
        WRITE
    }
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


pub mod uio {
    use core::{cmp::min, ffi::{c_int, c_void}, marker::PhantomData};
    use crate::{c_templates::uiomove, consts::error::Errno, types::{c_structs::Uio, c_types::c_ssize_t}};

    pub trait UioDirection {}
    pub struct UioRead;
    pub struct UioWrite;
    impl UioDirection for UioRead  {}
    impl UioDirection for UioWrite {}
    pub struct UioView<Direction: UioDirection> {
        ptr: *mut Uio,
        _direction_marker: PhantomData<Direction>
    }

    impl<Direction: UioDirection> UioView<Direction> {
        pub fn from_raw(ptr: *mut Uio) -> Self {
            Self { ptr, _direction_marker: PhantomData }
        }


        /*
         * Ok(size) => size of data moved to buf
         * Err(err) => error code of call to uiomove
         */
        fn uiomove_buf(&self, buf: &mut [u8]) -> Result<c_ssize_t, Errno> {
            let cp = buf.as_ptr() as *mut c_void;
            let n = buf.len() as c_int;
            let uio = self.ptr;
            let size_moved = min(
                buf.len() as c_ssize_t, 
                unsafe { self.ptr.read().uio_resid }
            );
            let error: c_int;

            unsafe { error = uiomove(cp, n, uio); }

            match error {
                0       => Ok(size_moved),
                errno   => Err(errno)
            }
        }
    }

    impl UioView<UioRead> {
        pub fn kernel_to_user_buf(&self, buf: &mut [u8]) -> Result<c_ssize_t, Errno> {
            self.uiomove_buf(buf)
        }
    }

    impl UioView<UioWrite> {
        pub fn user_to_kernel_buf(&self, buf: &mut [u8]) -> Result<c_ssize_t, Errno> {
            self.uiomove_buf(buf)
        }
    }
}
