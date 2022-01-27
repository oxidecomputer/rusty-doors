#![allow(clippy::needless_doctest_main)]

//! # Rusty Doors
//!
//! A streamlined and safe interface for creating and using Unix Doors.
//!
//! ## A simple door server
//!
//! ```no_run
//! use rusty_doors::{door_create, door_run};
//! use rusty_doors_macros::door;
//! use std::ffi::CString;
//!
//! #[derive(Default)]
//! #[repr(C)]
//! struct Wrapped {
//!     val: u64,
//!     other: u16,
//! }
//!
//! // To make a door handler just apply the #[door] attribute macro
//! #[door]
//! fn serv_proc(x: u64) -> Wrapped {
//!     let res = x + 47;
//!     println!("ARG: {}", x);
//!     return Wrapped {
//!         val: res,
//!         other: 99,
//!     };
//! }
//!
//! fn main() {
//!     let path = CString::new("/tmp/addr-test-door").expect("cstring");
//!     let fd = door_create(serv_proc);
//!     door_run(fd, path.as_c_str());
//! }
//! ```
//!
//! ## A door client
//!
//! ```no_run
//! use rusty_doors::door_call;
//! use std::fs::File;
//! use std::os::unix::io::AsRawFd;
//!
//! #[derive(Default)]
//! #[repr(C)]
//! struct Wrapped {
//!     val: u64,
//!     other: u16,
//! }
//!
//! fn main() {
//!     let file = File::open("/tmp/addr-test-door").expect("open fd");
//!
//!     let x: u64 = 74;
//!     let res: Wrapped = door_call(file.as_raw_fd(), x);
//!
//!     assert_eq!(res.val, 121);
//!     assert_eq!(res.other, 99);
//! }
//! ```

/// System level doors elements.
pub mod sys;

use crate::sys::{fattach, DoorArg};
use libc::{close, munmap, open, pause, unlink, O_CREAT, O_RDWR};
use std::alloc::{realloc, Layout};
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

/// Call the door referenced by `fd` with argument data `x`. Returns door call
/// result by value.
pub fn door_call<T, U: Default>(fd: c_int, x: T) -> U {
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

    res
}

/// Call the door referenced by `fd` with argument data `x`. The address pointed
/// to by `res` will be allocated by the door, and the result placed at that
/// location. The return value is the address to the pointed to address.
pub fn door_callp<T, U>(
    fd: c_int,
    x: T,
    mut res: ptr::NonNull<*mut U>,
) -> *mut U {
    unsafe {
        let res_ref = res.as_mut();
        let mut arg = DoorArg {
            data_ptr: (&x as *const T) as *mut c_char,
            data_size: size_of::<T>(),
            desc_ptr: ptr::null_mut(),
            desc_num: 0,
            rbuf: (*res_ref) as *mut c_char,
            rsize: size_of::<*mut U>(),
        };

        let _result = sys::door_call(fd, &mut arg);

        if (*res_ref) as *mut c_char != arg.rbuf {
            let newp = realloc(
                (*res_ref) as *mut u8,
                Layout::new::<U>(),
                arg.rsize as usize,
            );
            *res_ref = newp as *mut U;
            ptr::copy(
                arg.rbuf as *const u8,
                (*res_ref) as *mut u8,
                arg.rsize as usize,
            );
            munmap(arg.rbuf as *mut c_void, arg.rsize);
        }

        *res_ref
    }
}

/// Call the door referenced by `fd` with argument slice `x`. Returns doo call
/// result by value.
pub fn door_call_slice<T, U: Default>(fd: c_int, x: &[T]) -> U {
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

    res
}

/// Run a door server for the door handler referenced by `fd`. `fd` should be
/// the result of calling [`door_create`].
pub fn door_run(fd: i32, path: &CStr) -> ! {
    unsafe {
        let p = path.as_ptr();
        unlink(p);
        close(open(p, (O_CREAT | O_RDWR) as i32, 0x644));
        fattach(fd, p);
        loop {
            pause();
        }
    }
}

/// Create a door. The provided [`sys::DoorFunc`] will be called any time this
/// door is called. Returns a file descriptor reference the created door.
pub fn door_create(f: sys::DoorFunc) -> c_int {
    unsafe { sys::door_create(f, ptr::null_mut(), 0) }
}
