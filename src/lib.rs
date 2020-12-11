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
  CString::new(own_str(s1) + &own_str(s2)).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(string: *mut c_char) {
  if string.is_null() {
    return
  }
  unsafe { CString::from_raw(string) };
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
  len: usize,
  data: *mut RustPerson
}

impl Drop for RustPeople {
  fn drop(&mut self) {
    // Convert the pointer into a slice
    let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.len) };
    // Free the name of each person
    for person in s.iter() {
      unsafe { libc::free(person.name as *mut libc::c_void) };
    }
    // Free the people data
    unsafe {
      libc::free(self.data as *mut libc::c_void);
    }
  }
}

#[no_mangle]
pub extern "C" fn get_people() -> RustPeople {
  let a = CString::new(String::from("A")).unwrap();
  let b = CString::new(String::from("B")).unwrap();
  let c = CString::new(String::from("C")).unwrap();
  let d = CString::new(String::from("D")).unwrap();
  let mut people = vec![RustPerson { name: a.into_raw(),
                                     age: 24,
                                     height: 1.7 },
                        RustPerson { name: b.into_raw(),
                                     age: 52,
                                     height: 1.6 },
                        RustPerson { name: c.into_raw(),
                                     age: 53,
                                     height: 1.65 },
                        RustPerson { name: d.into_raw(),
                                     age: 23,
                                     height: 1.55 }].into_boxed_slice();
  let data = people.as_mut_ptr();
  let len = people.len();
  std::mem::forget(people);
  RustPeople { len, data }
}

#[no_mangle]
pub extern "C" fn free_people(people: RustPeople) { drop(people) }
