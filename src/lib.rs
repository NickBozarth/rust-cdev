#![no_std]

#[cfg(target_arch = "aarch64")]
::core::arch::global_asm!(include_str!("bti_aarch64.s"));


use ::core::{
    ffi::{c_int, c_void, c_char},
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

pub mod mutex;





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
    pub fn init_dev(
        cdevsw: &mut Cdevsw, 
        make_dev_args: &mut MakeDevArgs, 
        fmt: &mut *const c_char
    );
}

static mut CDEVSW: Cdevsw = Cdevsw::new(::core::ptr::null());
static mut CDEV: *mut Cdev = ::core::ptr::null_mut();



#[unsafe(no_mangle)]
pub extern "C" fn  rust_cdev_modevent(_module: *mut c_void, event: c_int, _arg: *mut c_void) -> c_int {
    match event {
        modeventtype::LOAD  => {
            if unsafe { !CDEV.is_null() } {
                return error::EBUSY;
            }

            let mut make_dev_args: MakeDevArgs =    MakeDevArgs::default();
            let mut fmt: *const c_char =            cstr!("%s");
            let cdevswstatic= unsafe { (&raw mut CDEVSW).as_mut_unchecked() };
            make_dev_args.mda_devsw =               &raw mut CDEVSW;

            unsafe { init_dev(cdevswstatic, &mut make_dev_args, &mut fmt); }

            if unsafe { CDEVSW.d_name.is_null() } {
                uprintf!(cstr!("Cdev modevent error: Cdevsw.d_name must not be null\n"));
                return error::EIO;
            }


            let error: c_int;
            unsafe {
                error = make_dev_s(
                    &raw mut make_dev_args,
                    &raw mut CDEV,
                    fmt
                );
            }

            match error {
                0 => {
                    uprintf!(cstr!("Cdev modevent: Created device {}\n"), CDEVSW.d_name);
                    error::NOERR
                }
                _ => {
                    uprintf!(cstr!("Cdev modevent error: Failed to create device {}\n"), error);
                    error::EIO
                }
            }
       },
       modeventtype::UNLOAD => {
            unsafe {
                match CDEV.is_null() {
                    true  => {
                        uprintf(cstr!("Cdev modevent: no device to unload\n"));
                        error::EIO
                    },
                    false => {
                        destroy_dev(CDEV);
                        uprintf(cstr!("Cdev modevent: destroyed dev\n"));
                        error::NOERR
                    },
                }
            }
       },
       _ => {
            uprintf!(cstr!("Cdev modevent error: Event not defined with type %i\n"), event);
            error::ENOTSUP
       }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
