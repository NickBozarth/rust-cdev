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

static mut CDEV: *mut Cdev = ::core::ptr::null_mut();



#[unsafe(no_mangle)]
pub extern "C" fn  rust_cdev_modevent(_module: *mut c_void, event: c_int, _arg: *mut c_void) -> c_int {
    match event {
        modeventtype::LOAD  => {
            if unsafe { !CDEV.is_null() } {
                return error::EBUSY;
            }

            let mut cdevsw: Cdevsw =                Cdevsw::new(::core::ptr::null());
            let mut make_dev_args: MakeDevArgs =    MakeDevArgs::default();
            let mut fmt: *const c_char =            cstr!("%s");
            make_dev_args.mda_devsw =               &raw mut cdevsw;

            unsafe { init_dev(&mut cdevsw, &mut make_dev_args, &mut fmt); }

            if cdevsw.d_name.is_null() {
                uprintf!(cstr!("Cdev modevent error: Cdevsw.d_name must not be null\n"));
                return error::EIO;
            }

            let error: c_int;
            error = 0;
            let mut cdev_attempt: *mut Cdev = ::core::ptr::null_mut();
            // unsafe {
            //     error = make_dev_s(
            //         &raw mut make_dev_args,
            //         &raw mut cdev_attempt,
            //         fmt,
            //     );
            // }

            match error {
                0 => {
                    uprintf!(cstr!("Cdev modevent: Created device {}\n"), cdevsw.d_name);
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
    };

    // unsafe {
    //     match event {
    //         0 => { // MOD_LOAD
    //
    //             init_dev(&CDEVSW, &MAKE_DEV_ARGS);
    //             let mut dev: *mut Cdev = ::core::ptr::null_mut();
    //
    //             let error = make_dev_p(
    //                 0,
    //                 &mut dev,
    //                 &raw mut RUST_CDEVSW,
    //                 ::core::ptr::null_mut(),
    //                 uid::ROOT,
    //                 gid::WHEEL,
    //                 file_mode::ROOT_READ | file_mode::ROOT_WRITE,
    //                 cstr!("%s"),
    //                 cstr!("rust_cdev")
    //             );
    //
    //             match error {
    //                 0 => {
    //                     RUST_CDEV = dev;
    //                     uprintf(cstr!("Created rust cdev\n"));
    //                     error::NOERR
    //                 },
    //                 _ => {
    //                     error::EIO
    //                 }
    //             }
    //         },
    //         1 => {
    //             if !RUST_CDEV.is_null() {
    //                 destroy_dev(RUST_CDEV);
    //                 RUST_CDEV = ::core::ptr::null_mut();
    //             }
    //             uprintf(cstr!("Destroyed rust cdev\n"));
    //             error::NOERR
    //         },
    //         _ => error::EOPNOTSUPP
    //     }
    // }


    0
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
