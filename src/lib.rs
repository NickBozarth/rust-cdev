#![no_std]

use ::core::{
    ffi::{c_int, c_void},
    panic::PanicInfo
};



pub mod types;
use types::c_structs::*;

pub mod consts;
use consts::*;

pub mod cdev;
use cdev::Cdevsw;

pub mod c_templates;
use c_templates::*;


#[macro_export]
macro_rules! cstr {
    ($lit:expr) => {
        $crate::cstr_raw!(core::concat!($lit, "\0"))
    };
}

#[macro_export]
macro_rules! cstr_raw {
    ($lit:expr) => {
        $lit.as_ptr() as *const ::core::ffi::c_char
    };
}




unsafe extern "Rust" { 
    pub static CDEVSW: Cdevsw;
}

pub static mut MAKE_DEV_ARGS: Option<MakeDevArgs> = None;




static mut RUST_CDEV: *mut Cdev = ::core::ptr::null_mut();



static mut RUST_CDEVSW: Cdevsw = Cdevsw::new(cstr!("rust_cdev"));


#[unsafe(no_mangle)]
pub extern "C" fn  rust_cdev_modevent(_module: *mut c_void, event: c_int, _arg: *mut c_void) -> c_int {
    unsafe {
        match event {
            0 => { // MOD_LOAD
                let mut dev: *mut Cdev = ::core::ptr::null_mut();

                let error = make_dev_p(
                    0,
                    &mut dev,
                    &raw mut RUST_CDEVSW,
                    ::core::ptr::null_mut(),
                    uid::ROOT,
                    gid::WHEEL,
                    file_mode::ROOT_READ | file_mode::ROOT_WRITE,
                    cstr!("%s"),
                    cstr!("rust_cdev")
                );

                match error {
                    0 => {
                        RUST_CDEV = dev;
                        uprintf(cstr!("Created rust cdev\n"));
                        error::NOERR
                    },
                    _ => {
                        error::EIO
                    }
                }
            },
            1 => {
                if !RUST_CDEV.is_null() {
                    destroy_dev(RUST_CDEV);
                    RUST_CDEV = ::core::ptr::null_mut();
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
