use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

/// # Safety
/// Dunno?
#[no_mangle]
pub unsafe extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

/// # Safety
/// Dunno?
#[no_mangle]
pub unsafe extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        let _ = CString::from_raw(s);
    }
}
