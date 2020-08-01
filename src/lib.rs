extern crate libc;

use std::ffi::CString;
use std::mem::transmute;

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
