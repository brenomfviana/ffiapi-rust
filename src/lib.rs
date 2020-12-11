extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn print_rust() {
  println!("This function was called from a Rust library.");
}

#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 { a + b }

#[repr(C)]
pub struct RustPerson {
  name: *const c_char,
  age: u32,
  height: f32
}

impl Drop for RustPerson {
  fn drop(&mut self) { unsafe { libc::free(self.name as *mut libc::c_void) }; }
}

#[no_mangle]
pub extern "C" fn get_str() -> *mut c_char {
  let r_str = CString::new(String::from("Rust")).unwrap();
  r_str.into_raw()
}

fn own_str(str: *const c_char) -> String {
  let c_buf: *const c_char = str;
  let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
  let str_slice: &str = c_str.to_str().unwrap();
  str_slice.to_owned()
}

#[no_mangle]
pub extern "C" fn concatenate(s1: *mut c_char, s2: *mut c_char) -> *mut c_char {
  let s1 = own_str(s1);
  let s2 = own_str(s2);
  CString::new(s1 + &s2).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(string: *mut c_char) {
  unsafe {
    if string.is_null() {
      return
    }
    CString::from_raw(string)
  };
}

#[no_mangle]
pub extern "C" fn get_person() -> RustPerson {
  let r_str = CString::new(String::from("Breno")).unwrap();
  RustPerson { name: r_str.into_raw(), age: 24, height: 1.7 }
}

#[no_mangle]
pub extern "C" fn free_person(person: RustPerson) { drop(person) }

#[repr(C)]
pub struct RustPeople {
  size: usize,
  list: *const RustPerson
}

impl Drop for RustPeople {
  fn drop(&mut self) { unsafe { libc::free(self.list as *mut libc::c_void) }; }
}

#[no_mangle]
pub extern "C" fn get_people() -> RustPeople {
  let mut list: Vec<RustPerson> = vec![];
  let names = ["A", "B", "C", "D"];
  let ages = [24, 52, 51, 23];
  let heights = [1.7, 1.6, 1.65, 1.55];
  for i in 0..4 {
    let r_str = CString::new(names[i]).unwrap();
    list.push(RustPerson { name: r_str.into_raw(),
                           age: ages[i],
                           height: heights[i] });
  }
  RustPeople { size: names.len(), list: (&*list).as_ptr() }
}

#[no_mangle]
pub extern "C" fn free_people(people: RustPeople) { drop(people) }
