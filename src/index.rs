use clang_sys;

bitflags! {
    pub struct Options: u32 {
        const EXCLUDE_DECLARATIONS_FROM_PCH = 0b00000001;
        const DISPLAY_DIAGNOSTICS           = 0b00000010;
    }
}

pub struct Index {
    ptr: clang_sys::CXIndex
}

impl Index {
    pub fn create(o: Options) -> Index {
        let exclude_decls_from_pch =
            if o.contains(EXCLUDE_DECLARATIONS_FROM_PCH) { 1 } else { 0 };
        let display_diagnostics =
            if o.contains(DISPLAY_DIAGNOSTICS) { 1 } else { 0 };

        unsafe {
            Index {
                ptr: clang_sys::clang_createIndex(
                    exclude_decls_from_pch,
                    display_diagnostics)
            }
        }
    }

    pub fn as_ptr(&self) -> clang_sys::CXIndex {
        self.ptr
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            clang_sys::clang_disposeIndex(self.ptr);
        }
    }
}