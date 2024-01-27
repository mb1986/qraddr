use libc::{c_int, c_uchar, dlsym, RTLD_NEXT};
use std::ffi::CString;

type QRegisterResourceDataType = unsafe extern "cdecl" fn(
    c_int,
    tree: *const c_uchar,
    name: *const c_uchar,
    data: *const c_uchar,
);

#[no_mangle]
pub extern "C" fn _Z21qRegisterResourceDataiPKhS0_S0_(
    version: c_int,
    tree: *const c_uchar,
    name: *const c_uchar,
    data: *const c_uchar,
) {
    println!("qraddr: {version},{tree:?},{name:?},{data:?}");

    let orig_fun_name = CString::new("_Z21qRegisterResourceDataiPKhS0_S0_").unwrap();
    let orig_fun_addr = unsafe { dlsym(RTLD_NEXT, orig_fun_name.as_ptr()) };
    let orig_fun: QRegisterResourceDataType = unsafe { std::mem::transmute(orig_fun_addr) };

    unsafe {
        orig_fun(version, tree, name, data);
    }
}
