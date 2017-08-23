use std::ffi;
use std::string;

pub fn cstring_from_str(s: &str) -> ffi::CString {
    ffi::CString::new(s).unwrap()
}

pub fn i32_try_from_usize(x: usize) -> Result<i32, string::String> {
    if x <= i32::max_value() as usize {
        Ok(x as i32)
    } else {
        Err(string::String::from("Conversion failed."))
    }
}

pub fn i32_from_usize(x: usize) -> i32 {
    i32_try_from_usize(x).unwrap()
}