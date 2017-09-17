use tu;

use clang_sys;
use std::ptr;

pub struct SourceLoc {
    pub file: tu::File,
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

impl SourceLoc {
    pub fn new(file: tu::File, line: u32, column: u32, offset: u32) -> SourceLoc {
        SourceLoc {
            file: file,
            line: line,
            column: column,
            offset: offset,
        }
    }
}

pub struct SourceLocation {
    obj: clang_sys::CXSourceLocation,
}

impl SourceLocation {
    pub fn from_obj(obj: clang_sys::CXSourceLocation) -> Self {
        SourceLocation { obj: obj }
    }

    pub fn file_location(&self) -> SourceLoc {
        let mut file_ptr: clang_sys::CXFile = ptr::null_mut();
        let mut line: u32 = 0;
        let mut column: u32 = 0;
        let mut offset: u32 = 0;

        unsafe {
            clang_sys::clang_getFileLocation(
                self.obj,
                &mut file_ptr,
                &mut line,
                &mut column,
                &mut offset,
            )
        };
        SourceLoc::new(tu::File::from_ptr(file_ptr), line, column, offset)
    }

    pub fn expansion_location(&self) -> SourceLoc {
        let mut file_ptr: clang_sys::CXFile = ptr::null_mut();
        let mut line: u32 = 0;
        let mut column: u32 = 0;
        let mut offset: u32 = 0;

        unsafe {
            clang_sys::clang_getExpansionLocation(
                self.obj,
                &mut file_ptr,
                &mut line,
                &mut column,
                &mut offset,
            )
        };
        SourceLoc::new(tu::File::from_ptr(file_ptr), line, column, offset)
    }

    pub fn spelling_location(&self) -> SourceLoc {
        let mut file_ptr: clang_sys::CXFile = ptr::null_mut();
        let mut line: u32 = 0;
        let mut column: u32 = 0;
        let mut offset: u32 = 0;

        unsafe {
            clang_sys::clang_getSpellingLocation(
                self.obj,
                &mut file_ptr,
                &mut line,
                &mut column,
                &mut offset,
            )
        };
        SourceLoc::new(tu::File::from_ptr(file_ptr), line, column, offset)
    }
}
