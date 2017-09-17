#[macro_use]
extern crate bitflags;
extern crate clang_sys;
extern crate libc;

pub mod cursor;
pub mod diagnostic;
pub mod index;
pub mod loc;
pub mod tu;
mod util;

use std::ffi;
use std::str;

pub struct String {
    obj: clang_sys::CXString
}

impl String {
    pub unsafe fn as_ptr(&self) -> *const libc::c_char {
        clang_sys::clang_getCString(self.obj)
    }

    pub fn as_cstr(&self) -> &ffi::CStr {
        unsafe {
            ffi::CStr::from_ptr(self.as_ptr())
        }
    }

    pub fn to_str(&self) -> &str {
        match self.as_cstr().to_str() {
            Ok(s) => s,
            Err(_) => "<Utf8Error>"
        }
    }
}

impl From<clang_sys::CXString> for String {
    fn from(s: clang_sys::CXString) -> String {
        String { obj: s }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            clang_sys::clang_disposeString(self.obj);
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    Success,
    Failure,
    Crashed,
    InvalidArguments,
    ASTReadError,
    UnknownError,
}

impl ErrorCode {
    pub fn description(&self) -> &str {
        match *self {
            ErrorCode::Success => "Success.",
            ErrorCode::Failure => "Generic error code.",
            ErrorCode::Crashed => "libclang crashed.",
            ErrorCode::InvalidArguments => "Invalid arguments.",
            ErrorCode::ASTReadError => "AST deserialization error.",
            ErrorCode::UnknownError => "Unknown error.",
        }
    }
}

impl From<clang_sys::CXErrorCode> for ErrorCode {
    fn from(e: clang_sys::CXErrorCode) -> ErrorCode {
        match e {
            clang_sys::CXError_Success => ErrorCode::Success,
            clang_sys::CXError_Failure => ErrorCode::Failure,
            clang_sys::CXError_Crashed => ErrorCode::Crashed,
            clang_sys::CXError_InvalidArguments => ErrorCode::InvalidArguments,
            clang_sys::CXError_ASTReadError => ErrorCode::ASTReadError,
            _ => ErrorCode::UnknownError,
        }
    }
}
