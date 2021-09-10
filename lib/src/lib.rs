pub mod sys;

use std::ptr;
use std::mem::size_of;
use std::os::raw::{
    c_char,
    c_int,
};
use std::ffi::CStr;
use crate::sys::{
    door_arg_t, 
    open,
    size_t,
    door_desc_t,
    uint_t,
    close,
    unlink,
    fattach,
    pause,
    O_RDWR, 
    O_CREAT,
};

pub type DoorFunc = unsafe extern "C" fn(
    arg1: *mut ::std::os::raw::c_void,
    arg2: *mut ::std::os::raw::c_char,
    arg3: size_t,
    arg4: *mut door_desc_t,
    arg5: uint_t,
);

pub fn door_call<T,U: Default>(fd: c_int, x: T) -> U {

    let mut res: U = U::default();

    let mut arg = door_arg_t{
        data_ptr: (& x as *const T) as *mut c_char,
        data_size: size_of::<T>() as u64,
        desc_ptr: ptr::null_mut(),
        desc_num: 0,
        rbuf: (&mut res as *mut U) as *mut c_char,
        rsize: size_of::<U>() as u64,
    };

    let _result = unsafe { sys::door_call(fd, &mut arg) };

    return res

}

pub fn door_run(fd: i32, path: &CStr) -> ! {

    unsafe {
        let p = path.as_ptr();
        unlink(p);
        close(open(p, (O_CREAT | O_RDWR) as i32, 0x644));
        fattach(fd, p);
        loop { pause(); }
    }

}

pub fn door_create(f: DoorFunc) -> c_int {

    unsafe {
        sys::door_create(Some(f), ptr::null_mut(), 0)
    }

}
