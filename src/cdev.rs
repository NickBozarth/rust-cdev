use ::core::ffi::{c_int, c_char, c_void};

use crate::types::d_functions::*;
use crate::consts::driver_version;


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
            d_spare0: ::core::ptr::null_mut(),
            d_kqfilter: None,
            d_purge: None,
            d_mmap_single: None,

            d_spare1: [0; 3],
            d_spare2: [core::ptr::null_mut(); 3],

            d_devs_lh: ::core::ptr::null_mut(),
            d_spare3: 0,
            __d_giant: ::core::ptr::null_mut(),
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

unsafe impl Send for Cdevsw {}
