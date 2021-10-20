pub mod sys;

use std::ptr;
use std::alloc::{realloc, Layout};
use std::mem::size_of;
use std::os::raw::{
    c_char,
    c_int,
    c_void,
};
use std::ffi::CStr;
use crate::sys::{
    DoorArg, 
    fattach,
};
use libc::{
    munmap,
    unlink,
    close,
    pause,
    open,
    O_RDWR, 
    O_CREAT,
};



pub fn door_call<T,U: Default>(fd: c_int, x: T) -> U {

    let mut res: U = U::default();

    let mut arg = DoorArg {
        data_ptr: (&x as *const T) as *mut c_char,
        data_size: size_of::<T>(),
        desc_ptr: ptr::null_mut(),
        desc_num: 0,
        rbuf: (&mut res as *mut U) as *mut c_char,
        rsize: size_of::<U>(),
    };

    let _result = unsafe { sys::door_call(fd, &mut arg) };

    return res

}

pub fn door_callp<T,U>(fd: c_int, x: T, res: *mut *mut U) -> *mut U {

    unsafe{

        let mut arg = DoorArg {
            data_ptr: (&x as *const T) as *mut c_char,
            data_size: size_of::<T>(),
            desc_ptr: ptr::null_mut(),
            desc_num: 0,
            rbuf: (*res) as *mut c_char,
            rsize: size_of::<*mut U>(),
        };

        let _result = sys::door_call(fd, &mut arg);

        if (*res) as *mut c_char != arg.rbuf {
            let newp = realloc(
                (*res) as *mut u8,
                Layout::new::<U>(),
                arg.rsize as usize,
            );
            *res = newp as *mut U;
            ptr::copy(
                arg.rbuf as *const u8,
                (*res) as *mut u8,
                arg.rsize as usize,
            );
            munmap(arg.rbuf as *mut c_void, arg.rsize);
        }


        return *res;

    }

}

pub fn door_call_slice<T,U: Default>(fd: c_int, x: &[T]) -> U {

    let mut res: U = U::default();

    let mut arg = DoorArg {
        data_ptr: (&x[0] as *const T) as *mut c_char,
        data_size: size_of::<T>() * x.len(),
        desc_ptr: ptr::null_mut(),
        desc_num: 0,
        rbuf: (&mut res as *mut U) as *mut c_char,
        rsize: size_of::<U>(),
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

pub fn door_create(f: sys::DoorFunc) -> c_int {

    unsafe {
        sys::door_create(f, ptr::null_mut(), 0)
    }

}
