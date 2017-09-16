use cursor;
use diagnostic;
use index;
use util;

use clang_sys;
use std::ptr;

bitflags! {
    pub struct Flags: u32 {
        const NONE                                          = 0x00;
        const DETAILED_PREPROCESSING_RECORD                 = 0x01;
        const INCOMPLETE                                    = 0x02;
        const PRECOMPILED_PREAMBLE                          = 0x04;
        const CACHE_COMPLETION_RESULTS                      = 0x08;
        const FOR_SERIALIZATION                             = 0x10;
        const CXX_CHAINED_PCH                               = 0x20;
        const SKIP_FUNCTION_BODIES                          = 0x40;
        const INCLUDE_BRIEF_COMMENTS_IN_CODE_COMPLETIONS    = 0x80;
        // #[cfg(feature="gte_clang_3_8")]
        // const CREATE_PREAMBLE_ON_FIRST_PARSE                = 0x100;
        // #[cfg(feature="gte_clang_3_9")]
        // const KEEP_GOING                                    = 0x200;
    }
}

pub struct TranslationUnit {
    ptr: clang_sys::CXTranslationUnit,
}

impl TranslationUnit {
    pub fn from_ptr(ptr: clang_sys::CXTranslationUnit) -> TranslationUnit {
        TranslationUnit { ptr: ptr }
    }

    pub fn parse<'a, I>(
        index: &index::Index,
        source_filename: &str,
        command_line_args: I,
        // unsaved_file: ...
        flags: Flags,
    ) -> Result<TranslationUnit, ::ErrorCode>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let source_filename_cstr = util::cstring_from_str(source_filename);
        let command_line_args_cstr_vec: Vec<_> =
            command_line_args.into_iter().map(|s| { util::cstring_from_str(s) }).collect();
        let command_line_args_ptr_vec: Vec<_> =
            command_line_args_cstr_vec.iter().map(|s| s.as_ptr()).collect();

        let mut tu_ptr: clang_sys::CXTranslationUnit = ptr::null_mut();

        let result = unsafe {
            clang_sys::clang_parseTranslationUnit2(
                index.as_ptr(),
                source_filename_cstr.as_ptr(),
                command_line_args_ptr_vec.as_ptr(),
                util::i32_from_usize(command_line_args_ptr_vec.len()),
                ptr::null_mut(),
                0,
                clang_sys::CXTranslationUnit_Flags::from_bits_truncate(flags.bits),
                &mut tu_ptr,
            )
        };

        let error_code = ::ErrorCode::from(result);

        match error_code {
            ::ErrorCode::Success => Ok(TranslationUnit::from_ptr(tu_ptr)),
            _ => Err(error_code),
        }
    }

    pub fn as_ptr(&self) -> clang_sys::CXTranslationUnit {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> &mut clang_sys::CXTranslationUnit {
        &mut self.ptr
    }

    pub fn get_cursor(&self) -> cursor::Cursor {
        unsafe {
            cursor::Cursor::from_obj(clang_sys::clang_getTranslationUnitCursor(self.ptr))
        }
    }

    pub fn get_diagnostics(&self) -> diagnostic::DiagnosticIterator {
        let num_diagnostics = unsafe {
            clang_sys::clang_getNumDiagnostics(self.ptr)
        };
        diagnostic::DiagnosticIterator::new(self, num_diagnostics)
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe {
            clang_sys::clang_disposeTranslationUnit(self.ptr);
        }
    }
}