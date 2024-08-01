// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::os::raw::{c_char, c_int, c_uint, c_ulonglong, c_void};

/// Structure used to pass/return to a [`door_call`] function.
#[repr(C)]
pub struct DoorArg {
    pub data_ptr: *mut c_char,
    pub data_size: usize,
    pub desc_ptr: *mut DoorDesc,
    pub desc_num: c_uint,
    pub rbuf: *mut c_char,
    pub rsize: usize,
}

/// Structure used to pass descriptors/objects in door invocations
#[repr(C)]
pub struct DoorDesc {
    pub d_attributes: c_uint,
    pub d_data: DoorData,
}

/// Underlying filesystem object definition
#[repr(C)]
pub union DoorData {
    pub d_desc: DDesc,
    pub d_resv: [c_int; 5usize],
}

/// Underlying file descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDesc {
    pub d_descriptor: c_int,
    pub d_id: c_ulonglong,
}

/// Function signature for door invocations
pub type DoorFunc = unsafe extern "C" fn(
    *mut c_void,
    *mut c_char,
    usize,
    *mut DoorDesc,
    c_uint,
);

#[cfg_attr(target_os = "illumos", link(name = "door"))]
#[cfg(target_os = "illumos")]
extern "C" {
    /// The `door_create` function creates a door descriptor that describes the
    /// procedure specified by the function `server_procedure`.
    ///
    /// See `man door_create` for more details.
    pub fn door_create(
        server_procedure: DoorFunc,
        cookie: *mut c_void,
        attributes: c_uint,
    ) -> c_int;

    /// `The door_call` function invokes the function associated with the door
    /// descriptor d, and passes the arguments (if any) specified in params.
    ///
    /// See `man door_call` for more details.
    pub fn door_call(d: c_int, params: *mut DoorArg) -> c_int;

    /// The `fattach` function attaches a STREAMS- or doors-based file
    /// descriptor to an object in the file system name space
    ///
    /// See `man fattach` for more details
    pub fn fattach(fildes: c_int, path: *const c_char) -> c_int;

    /// The `door_return` function returns from a door invocation.
    ///
    /// See `man door_return` for more details.
    pub fn door_return(
        data_ptr: *mut c_char,
        data_size: usize,
        desc_ptr: *mut DoorDesc,
        num_desc: c_uint,
    ) -> c_int;
}

#[cfg(not(target_os = "illumos"))]
mod stubs {
    use super::{DoorArg, DoorDesc, DoorFunc};
    use std::os::raw::{c_char, c_int, c_uint, c_void};

    // Stub out missing functions if on non-supported platform

    pub unsafe fn door_create(
        _server_procedure: DoorFunc,
        _cookie: *mut c_void,
        _attributes: c_uint,
    ) -> c_int {
        panic!("doors not supported on this platform");
    }

    pub unsafe fn door_call(_d: c_int, _params: *mut DoorArg) -> c_int {
        panic!("doors not supported on this platform");
    }

    pub unsafe fn fattach(_fildes: c_int, _path: *const c_char) -> c_int {
        panic!("doors not supported on this platform");
    }

    pub unsafe fn door_return(
        _data_ptr: *mut c_char,
        _data_size: usize,
        _desc_ptr: *mut DoorDesc,
        _num_desc: c_uint,
    ) -> c_int {
        panic!("doors not supported on this platform");
    }
}
#[cfg(not(target_os = "illumos"))]
pub use stubs::*;
