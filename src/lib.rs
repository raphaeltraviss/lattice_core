use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn lc_init() -> *mut c_char {
  let s = "Hello from Rust".to_string();
  let cs = CString::new(s).unwrap();
  cs.into_raw()
}

#[no_mangle]
pub extern "C" fn lc_free(s: *mut c_char) {
  unsafe {
    if s.is_null() { return }
    drop(CString::from_raw(s))
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
