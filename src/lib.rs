extern crate libc;

use std::ffi::CString;

#[repr(C)]
pub struct RustPerson {
  name: *const libc::c_char,
  age: u32,
  height: f32
}

impl Drop for RustPerson {
  fn drop(&mut self) { unsafe { libc::free(self.name as *mut libc::c_void) }; }
}

#[repr(C)]
pub struct RustPeople {
  size: usize,
  list: *const RustPerson
}

impl Drop for RustPeople {
  fn drop(&mut self) { unsafe { libc::free(self.list as *mut libc::c_void) }; }
}

#[no_mangle]
pub extern "C" fn rust() {
  println!("This function was called from a Rust library.");
}

#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 { a + b }

#[no_mangle]
pub extern "C" fn get_string() -> *const libc::c_char {
  let r_str = CString::new(String::from("Rust")).unwrap();
  r_str.into_raw()
}

#[no_mangle]
pub extern "C" fn get_person() -> RustPerson {
  let r_str = CString::new(String::from("Breno")).unwrap();
  RustPerson { name: r_str.into_raw(), age: 24, height: 1.7 }
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
