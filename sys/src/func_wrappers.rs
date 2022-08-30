extern crate static_assertions;

use std::{ffi::{self, CStr}, mem, marker::PhantomData};

use crate::{bindings::*, struct_wrappers::VMWrapper, helpers::*};

pub fn wren_version() -> i32 {
    unsafe {
      wrenGetVersionNumber() as i32
    }
}

pub trait RustWriteFn<T> where Self: Fn(&VMWrapper<T>, &str) + ZeroSized {}

impl<F, T> RustWriteFn<T> for F where F: Fn(&VMWrapper<T>, &str) + ZeroSized {}

pub fn wrap_write_fn<T, F: RustWriteFn<T>>(_: F) -> WrenWriteFn
{
    //assert!(mem::size_of::<F>() == 0);

    Some(wrapped_write::<T, F>)
}

unsafe extern fn wrapped_write<T, F: RustWriteFn<T>>(vm: *mut WrenVM, text: *const ffi::c_char) {
    let wrap: VMWrapper<T> = VMWrapper {
        user_data: PhantomData,
        vm
    };

    mem::zeroed::<F>()(&wrap, CStr::from_ptr(text).to_str().unwrap());
}
