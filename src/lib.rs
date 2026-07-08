#![no_std]

// Force the compiler to embed the GNU BTI note property into the final object file
// TODO ensure that other architectures are supported
#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    ".section .note.gnu.property, \"a\"",
    ".p2align 3",
    ".word 4",           // Name size (Length of "GNU\0")
    ".word 16",          // Data size
    ".word 5",           // Type (NT_GNU_PROPERTY_TYPE_0)
    ".asciz \"GNU\"",    // Owner Name
    
    // Property: GNU_PROPERTY_AARCH64_FEATURE_1_AND
    ".word 0xc0000000",  // pr_type
    ".word 4",           // pr_datasz
    ".word 1",           // pr_data (GNU_PROPERTY_AARCH64_FEATURE_1_BTI = 0x1)
    ".p2align 3"
);






use core::{
    ffi::{c_int, c_void, c_char, c_ulong, c_uint},
    panic::PanicInfo
};

/*
 * this is the core::ffi::c_size_t definition but it is unstable in the official core::ffi
 * typing is guaranteed to be safe on my compiling platform
 */
#[allow(non_camel_case_types)]
mod c_types {
    use core::ffi::c_char;
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
}

use c_types::*;

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



macro_rules! cstr {
    ($lit:expr) => {
        cstr_raw!(core::concat!($lit, "\0"))
    };
}

macro_rules! cstr_raw {
    ($lit:expr) => {
        $lit.as_ptr() as *const c_char
    };
}




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






#[repr(C)]
pub struct Cdevsw {
    pub d_version:      c_int,
    pub d_flags:        c_int,
    pub d_name:         *const c_char,
    pub d_open:         Option<DOpenT>,
    pub d_fdopen:       Option<DFdopenT>,
    pub d_close:        Option<DCloseT>,
    pub d_read:         Option<DReadT>,
    pub d_write:        Option<DWriteT>,
    pub d_ioctl:        Option<DIoctlT>,
    pub d_poll:         Option<DPollT>,
    pub d_mmap:         Option<DMmapT>,
    pub d_strategy:     Option<DStrategyT>,
    pub d_spare0:       *mut c_void,
    pub d_kqfilter:     Option<DKqfilterT>,
    pub d_purge:        Option<DPurgeT>,
    pub d_mmap_single:  Option<DMmapSingleT>,
    
    pub d_spare1:       [i32; 3],
    pub d_spare2:       [*mut c_void; 3],

    /* These fields should not be messed with by drivers */
    d_devs_lh:          *mut c_void,
    d_spare3:           c_int,
    __d_giant:          *mut c_void,
}


impl Cdevsw {
    pub const fn new(name: *const c_char) -> Self {
        Self {
            d_version: driver_version::CURRENT,
            d_flags: 0,
            d_name: name,
            d_open: None,
            d_fdopen: None,
            d_close: None,
            d_read: None,
            d_write: None,
            d_ioctl: None,
            d_poll: None,
            d_mmap: None,
            d_strategy: None,
            d_spare0: core::ptr::null_mut(),
            d_kqfilter: None,
            d_purge: None,
            d_mmap_single: None,

            d_spare1: [0; 3],
            d_spare2: [core::ptr::null_mut(); 3],

            d_devs_lh: core::ptr::null_mut(),
            d_spare3: 0,
            __d_giant: core::ptr::null_mut(),
        }
    }

    pub const fn with_version(mut self, version: c_int) -> Self             { self.d_version = version;             self }
    pub const fn with_flags(mut self, flags: c_int) -> Self         { self.d_flags = flags;                 self }

    pub const fn with_open(mut self, handler: DOpenT) -> Self               { self.d_open = Some(handler);          self }
    pub const fn with_fdopen(mut self, handler: DFdopenT) -> Self           { self.d_fdopen = Some(handler);        self }
    pub const fn with_close(mut self, handler: DCloseT) -> Self             { self.d_close = Some(handler);         self }
    pub const fn with_read(mut self, handler: DReadT) -> Self               { self.d_read = Some(handler);          self }
    pub const fn with_write(mut self, handler: DWriteT) -> Self             { self.d_write = Some(handler);         self }
    pub const fn with_ioctl(mut self, handler: DIoctlT) -> Self             { self.d_ioctl = Some(handler);         self }
    pub const fn with_poll(mut self, handler: DPollT) -> Self               { self.d_poll = Some(handler);          self }
    pub const fn with_mmap(mut self, handler: DMmapT) -> Self               { self.d_mmap = Some(handler);          self }
    pub const fn with_strategy(mut self, handler: DStrategyT) -> Self       { self.d_strategy = Some(handler);      self }
    pub const fn with_kqfilter(mut self, handler: DKqfilterT) -> Self       { self.d_kqfilter = Some(handler);      self }
    pub const fn with_purge(mut self, handler: DPurgeT) -> Self             { self.d_purge = Some(handler);         self }
    pub const fn with_mmap_single(mut self, handler: DMmapSingleT) -> Self  { self.d_mmap_single = Some(handler);   self }

    pub const fn with_spare0(mut self, ptr: *mut c_void) -> Self            { self.d_spare0 = ptr;                  self }
    pub const fn with_spare1(mut self, spare: [i32; 3]) -> Self             { self.d_spare1 = spare;                self }
    pub const fn with_spare2(mut self, spare: [*mut c_void; 3]) -> Self     { self.d_spare2 = spare;                self }
}



mod driver_version {
    /*
     * <sys/conf.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH A D_VERSION PREFIX
     * ex: driver_version::V00 -> D_VERSION_00 in docs
     */
    use core::ffi::c_int;

    pub const V00: c_int     = 0x20011966;
    pub const V01: c_int     = 0x17032005;
    pub const V02: c_int     = 0x28042009;
    pub const V03: c_int     = 0x17122009;
    pub const V04: c_int     = 0x5c48c353;
    pub const CURRENT: c_int = V04;
}

mod fflag {
    /*
     * <sys/fcntl.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH AN F PREFIX
     * ex: fflag::READ -> FREAD in docs
     */
    use core::ffi::c_int;

    pub const READ: c_int       = 0x0001;
    pub const WRITE: c_int      = 0x0002;
    pub const EXEC: c_int       = 0x00040000;
    pub const SEARCH: c_int     = 0x00040000;
    pub const HASLOCK: c_int    = 0x4000;
    pub const LASTCLOSE: c_int  = 0x00020000;
    pub const REVOKE: c_int     = 0x00200000;
    pub const OPENFAILED: c_int = 0x00080000;
    pub const KQALLOWED: c_int  = 0x00800000;
}

mod oflag {
    /*
     * <sys/fcntl.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH AN O_ PREFIX
     * ex: oflag::RDONLY -> O_RDONLY in docs
     */
    use core::ffi::c_int;

    pub const RDONLY: c_int             = 0x0000;
    pub const WRONLY: c_int             = 0x0001;
    pub const RDWR: c_int               = 0x0002;
    pub const ACCMODE: c_int            = 0x0003;

    pub const NONBLOCK: c_int           = 0x0004;
    pub const APPEND: c_int             = 0x0008;
    pub const SHLOCK: c_int             = 0x0010;
    pub const EXLOCK: c_int             = 0x0020;
    pub const ASYNC: c_int              = 0x0040;
    pub const FSYNC: c_int              = 0x0080;
    pub const SYNC: c_int               = 0x0080;
    pub const NOFOLLOW: c_int           = 0x0100;
    pub const CREAT: c_int              = 0x0200;
    pub const TRUNC: c_int              = 0x0400;
    pub const EXCL: c_int               = 0x0800;
    pub const NOCTTY: c_int             = 0x8000;
    pub const DIRECT: c_int             = 0x00010000;
    pub const DIRECTORY: c_int          = 0x00020000;
    pub const EXEC: c_int               = 0x00040000;
    pub const SEARCH: c_int             = EXEC;
    pub const TTY_INIT: c_int           = 0x00080000;
    pub const CLOEXEC: c_int            = 0x00100000;
    pub const VERIFY: c_int             = 0x00200000;
    pub const PATH: c_int               = 0x00400000;
    pub const RESOLVE_BENEATH: c_int    = 0x00800000;
    pub const DSYNC: c_int              = 0x01000000;
    pub const EMPTY_PATH: c_int         = 0x02000000;
    pub const NAMEDATTR: c_int          = 0x04000000;
    pub const XATTR: c_int              = NAMEDATTR;
    pub const CLOFORK: c_int            = 0x08000000;
}

mod ioflag {
    /*
     * <sys/fcntl.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH A IO_ PREFIX
     * ex: ioflag::UNIT -> IO_UNIT in docs
     */
    use core::ffi::c_int;

    pub const UNIT: c_int = 0x0001;
    pub const APPEND: c_int = 0x0002;
    pub const NDELAY: c_int = 0x0004;
    pub const NODELOCKED: c_int = 0x0008;
    pub const ASYNC: c_int = 0x0010;
    pub const VMIO: c_int = 0x0020;
    pub const INVAL: c_int = 0x0040;
    pub const SYNC: c_int = 0x0080;
    pub const EXT: c_int = 0x0400;
    pub const NORMAL: c_int = 0x0800;
    pub const NOMACCHECK: c_int = 0x1000;
}


mod error {
    /*
     * <sys/errno.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM EXACTLY AS IS HERE
     * ex: error::EPERM -> EPERM in docs
     * *note* NOERR is not contained in original definitions
     */
    use core::ffi::c_int;

    pub const NOERR: c_int            = 0;
    pub const EPERM: c_int            = 1;
    pub const ENOENT: c_int           = 2;
    pub const ESRCH: c_int            = 3;
    pub const EINTR: c_int            = 4;
    pub const EIO: c_int              = 5;
    pub const ENXIO: c_int            = 6;
    pub const E2BIG: c_int            = 7;
    pub const ENOEXEC: c_int          = 8;
    pub const EBADF: c_int            = 9;
    pub const ECHILD: c_int           = 10;
    pub const EDEADLK: c_int          = 11;
    pub const ENOMEM: c_int           = 12;
    pub const EACCES: c_int           = 13;
    pub const EFAULT: c_int           = 14;
    pub const ENOTBLK: c_int          = 15;
    pub const EBUSY: c_int            = 16;
    pub const EEXIST: c_int           = 17;
    pub const EXDEV: c_int            = 18;
    pub const ENODEV: c_int           = 19;
    pub const ENOTDIR: c_int          = 20;
    pub const EISDIR: c_int           = 21;
    pub const EINVAL: c_int           = 22;
    pub const ENFILE: c_int           = 23;
    pub const EMFILE: c_int           = 24;
    pub const ENOTTY: c_int           = 25;
    pub const ETXTBSY: c_int          = 26;
    pub const EFBIG: c_int            = 27;
    pub const ENOSPC: c_int           = 28;
    pub const ESPIPE: c_int           = 29;
    pub const EROFS: c_int            = 30;
    pub const EMLINK: c_int           = 31;
    pub const EPIPE: c_int            = 32;
    pub const EDOM: c_int             = 33;
    pub const ERANGE: c_int           = 34;
    pub const EAGAIN: c_int           = 35;
    pub const EWOULDBLOCK: c_int      = EAGAIN;
    pub const EINPROGRESS: c_int      = 36;
    pub const EALREADY: c_int         = 37;
    pub const ENOTSOCK: c_int         = 38;
    pub const EDESTADDRREQ: c_int     = 39;
    pub const EMSGSIZE: c_int         = 40;
    pub const EPROTOTYPE: c_int       = 41;
    pub const ENOPROTOOPT: c_int      = 42;
    pub const EPROTONOSUPPORT: c_int  = 43;
    pub const ESOCKTNOSUPPORT: c_int  = 44;
    pub const EOPNOTSUPP: c_int       = 45;
    pub const ENOTSUP: c_int          = EOPNOTSUPP;
    pub const EPFNOSUPPORT: c_int     = 46;
    pub const EAFNOSUPPORT: c_int     = 47;
    pub const EADDRINUSE: c_int       = 48;
    pub const EADDRNOTAVAIL: c_int    = 49;
    pub const ENETDOWN: c_int         = 50;
    pub const ENETUNREACH: c_int      = 51;
    pub const ENETRESET: c_int        = 52;
    pub const ECONNABORTED: c_int     = 53;
    pub const ECONNRESET: c_int       = 54;
    pub const ENOBUFS: c_int          = 55;
    pub const EISCONN: c_int          = 56;
    pub const ENOTCONN: c_int         = 57;
    pub const ESHUTDOWN: c_int        = 58;
    pub const ETOOMANYREFS: c_int     = 59;
    pub const ETIMEDOUT: c_int        = 60;
    pub const ECONNREFUSED: c_int     = 61;
    pub const ELOOP: c_int            = 62;
    pub const ENAMETOOLONG: c_int     = 63;
    pub const EHOSTDOWN: c_int        = 64;
    pub const EHOSTUNREACH: c_int     = 65;
    pub const ENOTEMPTY: c_int        = 66;
    pub const EPROCLIM: c_int         = 67;
    pub const EUSERS: c_int           = 68;
    pub const EDQUOT: c_int           = 69;
    pub const ESTALE: c_int           = 70;
    pub const EREMOTE: c_int          = 71;
    pub const EBADRPC: c_int          = 72;
    pub const ERPCMISMATCH: c_int     = 73;
    pub const EPROGUNAVAIL: c_int     = 74;
    pub const EPROGMISMATCH: c_int    = 75;
    pub const EPROCUNAVAIL: c_int     = 76;
    pub const ENOLCK: c_int           = 77;
    pub const ENOSYS: c_int           = 78;
    pub const EFTYPE: c_int           = 79;
    pub const EAUTH: c_int            = 80;
    pub const ENEEDAUTH: c_int        = 81;
    pub const EIDRM: c_int            = 82;
    pub const ENOMSG: c_int           = 83;
    pub const EOVERFLOW: c_int        = 84;
    pub const ECANCELED: c_int        = 85;
    pub const EILSEQ: c_int           = 86;
    pub const ENOATTR: c_int          = 87;
    pub const EDOOFUS: c_int          = 88;
    pub const EBADMSG: c_int          = 89;
    pub const EMULTIHOP: c_int        = 90;
    pub const ENOLINK: c_int          = 91;
    pub const EPROTO: c_int           = 92;
    pub const ENOTCAPABLE: c_int      = 93;
    pub const ECAPMODE: c_int         = 94;
    pub const ENOTRECOVERABLE: c_int  = 95;
    pub const EOWNERDEAD: c_int       = 96;
    pub const EINTEGRITY: c_int       = 97;
    pub const ELAST: c_int            = 97;
    pub const ERESTART: c_int         = -1;
    pub const EJUSTRETURN: c_int      = -2;
    pub const ENOIOCTL: c_int         = -3;
    pub const EDIRIOCTL: c_int        = -4;
    pub const ERELOOKUP: c_int        = -5;
}

mod uid {
    /*
     * <sys/conf.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH A UID_ PREFIX
     * ex: uid::ROOT -> UID_ROOT in docs
     */
    use super::c_types::c_uid_t;

    pub const ROOT: c_uid_t     = 0;
    pub const BIN: c_uid_t      = 3;
    pub const UUCP: c_uid_t     = 66;
    pub const NOBODY: c_uid_t   = 65534;
}

mod gid {
    /*
     * <sys/conf.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH A GID_ PREFIX
     * ex: gid::WHEEL -> GID_WHEEL in docs
     */
    use super::c_types::c_gid_t;

    pub const WHEEL: c_gid_t    = 0;
    pub const KMEM: c_gid_t     = 2;
    pub const TTY: c_gid_t      = 4;
    pub const OPERATOR: c_gid_t = 5;
    pub const BIN: c_gid_t      = 7;
    pub const GAMES: c_gid_t    = 13;
    pub const VIDEO: c_gid_t    = 44;
    pub const RT_PRIO: c_gid_t  = 47;
    pub const ID_PRIO: c_gid_t  = 48;
    pub const DIALER: c_gid_t   = 68;
    pub const U2F: c_gid_t      = 116;
    pub const NOGROUP: c_gid_t  = 65533;
    pub const NOBODY: c_gid_t   = 65534;
}

mod cdevsw_flag {
    /*
     * <sys/conf.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH A D_ PREFIX
     * DISREGARD THE device_type/behavior ABSTRACTION WHEN CHECKING DOCS
     * ex: cdevsw_flag::device_type::TAPE -> D_TAPE in docs
     */
    use core::ffi::c_int;

    /*
     * DEVICE TYPES ARE NOT NECESSARY, 0 IS THE DEFAULT
     * ONLY UP TO ONE TYPE FLAG MAY BE USED AT ONCE
     */
    pub mod device_type {
        use super::c_int;

        const TAPE: c_int   = 0x0001;
        const DISK: c_int   = 0x0002;
        const TTY: c_int    = 0x0004;
        const MEM: c_int    = 0x0008;
    }

    /*
     * BEHAVIOR FLAGS ARE NOT NECESSARY, 0 IS THE DEFAULT
     * BEHAVIOR FLAGS MAY BE CHAINED TOGETHER IF NEEDED
     */
    pub mod behavior {
        use super::c_int;

        const TRACKCLOSE: c_int = 0x00080000;
        const MMAP_ANON: c_int  = 0x00100000;
        const GIANTOK: c_int    = 0x00200000;
        const NEEDGIANT: c_int  = 0x00400000;
        const NEEDMINOR: c_int  = 0x00800000;
    }
}

mod makedev_flag {
    /*
     * <sys/conf.h>
     * ALL REFERENCES TO THESE IN C DOCUMENTATION REFER TO THEM WITH A MAKEDEV_ PREFIX
     * ex: makedev_flag::REF -> MAKEDEV_REF in docs
     */
    use core::ffi::c_int;

    pub const REF: c_int       = 0x01;
    pub const WHTOUT: c_int    = 0x02;
    pub const NOWAIT: c_int    = 0x04;
    pub const WAITOK: c_int    = 0x08;
    pub const ETERNAL: c_int   = 0x10;
    pub const CHECKNAME: c_int = 0x20;
}

mod file_mode {
    /*
     * THESE ARE NOT OFFICIAL TYPES
     * THESE TYPES ARE HERE TO MAKE FILE MODES MORE LEGIBLE
     * ex: 0o600 -> (ROOT_READ | ROOT_WRITE)
     */
    use core::ffi::c_int;

    pub const ROOT_READ: c_int        = 0b100_000_000;
    pub const ROOT_WRITE: c_int       = 0b010_000_000;
    pub const ROOT_EXECUTE: c_int     = 0b001_000_000;

    pub const GROUP_READ: c_int       = 0b000_100_000;
    pub const GROUP_WRITE: c_int      = 0b000_010_000;
    pub const GROUP_EXECUTE: c_int    = 0b000_001_000;

    pub const USER_READ: c_int        = 0b000_000_100;
    pub const USER_WRITE: c_int       = 0b000_000_010;
    pub const USER_EXECUTE: c_int     = 0b000_000_001;
}


unsafe extern "C" {
    fn uprintf(fmt: *const c_char, ...) -> c_int;


    fn make_dev_p(
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


    fn destroy_dev(dev: *mut Cdev);
}

static mut RUST_CDEV: *mut Cdev = core::ptr::null_mut();



static mut RUST_CDEVSW: Cdevsw = Cdevsw::new(cstr!("rust_cdev"));

unsafe extern "C" fn d_open(
    _cdev: *mut Cdev, 
    _fflag: c_int, 
    _devtype: c_int, 
    _td: *mut Thread
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn  rust_cdev_modevent(_module: *mut c_void, event: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        match event {
            0 => { // MOD_LOAD
                let mut dev: *mut Cdev = core::ptr::null_mut();

                let error = make_dev_p(
                    0,
                    &mut dev,
                    &raw mut RUST_CDEVSW,
                    core::ptr::null_mut(),
                    uid::ROOT,
                    gid::WHEEL,
                    file_mode::ROOT_READ | file_mode::ROOT_WRITE,
                    cstr!("%s"),
                    cstr!("rust_cdev")
                );


                if error != 0 {
                    error::EIO
                } else {
                    RUST_CDEV = dev;
                    uprintf(cstr!("Created rust cdev\n"));
                    error::NOERR
                }
            },
            1 => {
                if !RUST_CDEV.is_null() {
                    destroy_dev(RUST_CDEV);
                    RUST_CDEV = core::ptr::null_mut();
                }
                uprintf(cstr!("Destroyed rust cdev\n"));
                error::NOERR
            },
            _ => error::EOPNOTSUPP
        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
