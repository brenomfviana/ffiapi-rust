extern crate libc;

use std::ffi::CString;
use std::mem::transmute;

#[repr(C)]
pub struct RustPerson {
  name: *const libc::c_char,
  age: u32,
	height: f32
}

#[no_mangle]
pub extern "C" fn rust() {
  println!("This function was called from a Rust library.");
}

#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 { a + b }

#[no_mangle]
pub extern fn get_string() -> *const libc::c_char {
  let data: *const CString;
  unsafe {
    data = transmute(Box::new(CString::from_vec_unchecked(b"Rust!".to_vec())));
    return (&*data).as_ptr();
  }
}

#[no_mangle]
pub extern "C" fn get_person() -> RustPerson {
  let data: *const CString;
  unsafe {
    data = transmute(Box::new(CString::from_vec_unchecked(b"Breno".to_vec())));
  }
  RustPerson { name: unsafe { (&*data).as_ptr() }, age: 24, height: 1.7 }
}
