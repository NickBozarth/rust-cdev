/*
 * WIP UNIMPLEMENTED FEATURES
 * man mutex(9) FreeBSD 15.0-RELEASE
 */


use ::core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    ffi::{c_char, c_int},
    ops::{Deref, DerefMut},
    marker::PhantomData
};

use crate::{
    types::c_structs::Mtx,
    c_templates::{mtx_init, mtx_destroy, mtx_lock, mtx_unlock, mtx_initialized}
};

pub enum MutexError {
    AlreadyInitialized,
    Uninitialized,
    Poison,
}





/*
 * Negative impl being a feature forces the use of PhantomData to prevent send
 * Some threads require that any mutexes opened must be closed by the same thread
 */
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    _nosend_marker: PhantomData<*mut ()>
}

unsafe impl<T: Sync> Sync for MutexGuard<'_, T> {}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe { mtx_unlock(self.mutex.c_mutex.get()); }
    }
}



pub struct Mutex<T> {
    c_mutex:        UnsafeCell<Mtx>,
    data:           UnsafeCell<T>,
    is_poisoned:    AtomicBool,
}


/*
 * Public functions should mirror std::mutex interface
 *  or be distinguished otherwise (none implemented yet)
 */
impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            c_mutex:        UnsafeCell::new(Mtx::new()),
            data:           UnsafeCell::new(data),
            is_poisoned:    AtomicBool::new(false),
            // init_opts: AtomicI32 TODO potentially add this and expand support for all mutex opts
        }
    }

    /*
     * opts can be found in crate::types::mutex::init
     */
    pub fn init(
        &self, 
        name: *const c_char, 
        mtype: *const c_char, 
        opts: c_int
    ) -> Result<(), MutexError> {
        

        unsafe {
            mtx_init(
                self.c_mutex.get(),
                name,
                mtype,
                opts
            );
        }

        Ok(())
    }

    pub fn clear_poison() {}
    pub fn data_ptr() {}
    pub fn get_cloned() {}
    pub fn get_mut() {}
    pub fn into_inner() {}
    pub fn is_poisoned() {}
    pub fn lock(&self) -> Result<MutexGuard<'_, T>, MutexError> {
        self.check_alive()?;

        unsafe { mtx_lock(self.c_mutex.get()); }

        Ok(
            MutexGuard { 
                mutex: self, 
                _nosend_marker: PhantomData 
            }
        )
    }
    pub fn replace() {}
    pub fn get() {}
    pub fn try_lock() {}






    fn is_initialized(&self) -> bool {
        let is_initialized: c_int;
        unsafe { is_initialized = mtx_initialized(self.c_mutex.get()); }
        is_initialized != 0
    }

    fn check_alive(&self) -> Result<(), MutexError> {
        if !self.is_initialized() {
            return Err(MutexError::Uninitialized)
        }
        if !self.is_poisoned.load(Ordering::Relaxed) {
            return Err(MutexError::Poison)
        }
        Ok(())
    }
}


unsafe impl<T: Send> Sync for Mutex<T> {}
unsafe impl<T: Send> Send for Mutex<T> {}
