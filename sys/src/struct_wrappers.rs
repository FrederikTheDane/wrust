use std::{marker::PhantomData, ffi::CString, mem, io::Error};

use crate::{bindings::*, func_wrappers::*, defaults::default_write_fn, helpers::ZeroSized};

pub struct VMWrapper<T> {
    pub(crate) user_data: PhantomData<T>,
    pub(crate) vm: *mut WrenVM,
}

impl<T> VMWrapper<T> {
  pub fn from_configuration(config: ConfigurationWrapper<T>) -> Self {
    let vm: Option<&mut WrenVM>;

    unsafe {
      vm = wrenNewVM(config.config).as_mut();
    }
    
    Self {
      user_data: PhantomData,
      vm: vm.unwrap()
    }
  }

  pub fn run_gc(&mut self) {
    unsafe {
      wrenCollectGarbage(self.vm)
    }
  }

  pub fn interpret(&mut self, module: &str, source: &str) -> Result<WrenInterpretResult, Box<dyn std::error::Error>> {
    unsafe {
      let m = CString::new(module)?.into_raw();
      let s = CString::new(source)?.into_raw();

      let result = wrenInterpret(self.vm, m, s);

      drop(CString::from_raw(m));
      drop(CString::from_raw(s));

      Ok(result)
    }
  }
}

pub struct ConfigurationWrapper<'a, T> {
  user_data: PhantomData<&'a T>,
  config: &'a mut WrenConfiguration,
}

impl <T> ConfigurationWrapper<'_, T> 
{
  pub fn new() -> Self 
  {
    unsafe {
      let config_ptr: *mut WrenConfiguration = std::mem::MaybeUninit::uninit().as_mut_ptr();

      wrenInitConfiguration(config_ptr);

      let config = &mut *config_ptr;

      //|_: &VMWrapper<T>, text: &str| { println!("{}", text) }
      //default_write_fn::<T>

      config.writeFn = wrap_write_fn(default_write_fn::<T>);

      Self { 
        user_data: PhantomData, 
        config
      }
    }
  }
}