#![feature(box_syntax)]

extern crate libc;
extern crate rand;

mod machine;
mod cpu;

use machine::Machine;

use std::mem;

#[no_mangle]
pub unsafe extern "C" fn ax_new() -> *mut libc::c_void {
    mem::transmute(box Machine::new())
}

#[no_mangle]
pub unsafe extern "C" fn ax_delete(ptr: *mut libc::c_void) {
    let _drop_me: Box<Machine> = mem::transmute(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn ax_reset(ptr: *mut libc::c_void) {
    (*(ptr as *mut Machine)).reset();
}

#[no_mangle]
pub unsafe extern "C" fn ax_set_on_video_frame(ptr: *mut libc::c_void,
                                               cb: extern "C" fn(*mut libc::c_void,
                                                                 *const u8,
                                                                 u32,
                                                                 u32,
                                                                 u8)
                                                                 -> (),
                                               user_data: *mut libc::c_void) {
    (*(ptr as *mut Machine)).set_on_video_frame(cb, user_data);
}

#[no_mangle]
pub unsafe extern "C" fn ax_open_rom(ptr: *mut libc::c_void, filename: *const libc::c_char) {
    let filename = std::ffi::CStr::from_ptr(filename).to_str().unwrap();

    (*(ptr as *mut Machine)).open_rom(filename);
}

#[no_mangle]
pub unsafe extern "C" fn ax_run_next(ptr: *mut libc::c_void) {
    (*(ptr as *mut Machine)).run_next();
}
