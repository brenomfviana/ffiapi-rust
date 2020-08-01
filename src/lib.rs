#[no_mangle]
pub extern "C" fn rust() {
  println!("This function was called from a Rust library.");
}

#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 { a + b }
