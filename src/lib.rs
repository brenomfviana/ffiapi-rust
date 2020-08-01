extern crate libc;

use std::ffi::CString;
use std::mem::transmute;

#[repr(C)]
pub struct RustPerson {
  name: *const libc::c_char,
  age: u32,
  height: f32
}

#[repr(C)]
pub struct RustPeople {
  size: usize,
  list: *const RustPerson
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
    data = transmute(
      Box::new(CString::from_vec_unchecked(b"Breno".to_vec()))
    );
  }
  RustPerson { name: unsafe { (&*data).as_ptr() }, age: 24, height: 1.7 }
}

#[no_mangle]
pub extern "C" fn get_people() -> RustPeople {
  let mut list: Vec<RustPerson> = vec![];
  let names = ["Breno", "Solange", "Aurélio", "Larissa"];
  let ages = [24, 52, 51, 23];
  let heights = [1.7, 1.6, 1.65, 1.55];
  for i in 0..4 {
    let data: *const CString;
    unsafe {
      data = transmute(
        Box::new(CString::from_vec_unchecked(names[i].as_bytes().to_vec()))
      );
    }
    list.push(RustPerson {
      name: unsafe { (&*data).as_ptr() }, age: ages[i], height: heights[i]
    });
  }
  RustPeople { size: names.len(), list: (&*list).as_ptr() }
}
