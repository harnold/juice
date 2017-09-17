use loc;
use tu;

use clang_sys;

pub enum Severity {
    Ignored,
    Note,
    Warning,
    Error,
    Fatal
}

impl From<clang_sys::CXDiagnosticSeverity> for Severity {
    fn from(s: clang_sys::CXDiagnosticSeverity) -> Severity {
        match s {
            clang_sys::CXDiagnostic_Ignored => Severity::Ignored,
            clang_sys::CXDiagnostic_Note => Severity::Note,
            clang_sys::CXDiagnostic_Warning => Severity::Warning,
            clang_sys::CXDiagnostic_Error => Severity::Error,
            clang_sys::CXDiagnostic_Fatal => Severity::Fatal,
            _ => panic!("Invalid CXDiagnosticSeverity value.")
        }
    }
}

pub struct Diagnostic {
    ptr: clang_sys::CXDiagnostic
}

impl Diagnostic {
    pub fn from_ptr(ptr: clang_sys::CXDiagnostic) -> Diagnostic {
        Diagnostic { ptr: ptr }
    }

    pub fn severity(&self) -> Severity {
        let s = unsafe { clang_sys::clang_getDiagnosticSeverity(self.ptr) };
        Severity::from(s)
    }

    pub fn spelling(&self) -> ::String {
        let s = unsafe { clang_sys::clang_getDiagnosticSpelling(self.ptr) };
        ::String::from(s)
    }

    pub fn location(&self) -> loc::SourceLocation {
        let l = unsafe { clang_sys::clang_getDiagnosticLocation(self.ptr) };
        loc::SourceLocation::from_obj(l)
    }
}

impl Drop for Diagnostic {
    fn drop(&mut self) {
        unsafe {
            clang_sys::clang_disposeDiagnostic(self.ptr)
        }
    }
}

pub struct DiagnosticIterator<'a> {
    tu: &'a tu::TranslationUnit,
    num_diagnostics: u32,
    current: u32
}

impl<'a> DiagnosticIterator<'a> {
    pub fn new(tu: &'a tu::TranslationUnit, num_diagnostics: u32) -> DiagnosticIterator {
        DiagnosticIterator {
            tu: tu,
            num_diagnostics: num_diagnostics,
            current: 0
        }
    }
}

impl<'a> Iterator for DiagnosticIterator<'a> {
    type Item = Diagnostic;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.num_diagnostics {
            let diag_ptr = unsafe {
                clang_sys::clang_getDiagnostic(self.tu.as_ptr(), self.current)
            };
            self.current = self.current + 1;
            Some(Diagnostic::from_ptr(diag_ptr))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.num_diagnostics - self.current) as usize;
        (remaining, Some(remaining))
    }
}

pub struct DiagnosticSet {
    ptr: clang_sys::CXDiagnosticSet
}

impl DiagnosticSet {
    pub fn from_ptr(ptr: clang_sys::CXDiagnosticSet) -> DiagnosticSet {
        DiagnosticSet { ptr: ptr }
    }
}

impl Drop for DiagnosticSet {
    fn drop(&mut self) {
        unsafe {
            clang_sys::clang_disposeDiagnosticSet(self.ptr)
        }
    }
}