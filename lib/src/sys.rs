// Copyright 2021 Oxide Computer Company

use std::os::raw::{
    c_char,
    c_uint,
    c_int,
    c_void,
    c_ulonglong,
};

#[repr(C)]
pub struct DoorArg {
    pub data_ptr: *mut c_char,
    pub data_size: usize,
    pub desc_ptr: *mut DoorDesc,
    pub desc_num: c_uint,
    pub rbuf: *mut c_char,
    pub rsize: usize,
}

#[repr(C)]
pub struct DoorDesc {
    pub d_attributes: c_uint,
    pub d_data: DoorData
}

#[repr(C)]
pub union DoorData {
    pub d_desc: DDesc,
    pub d_resv: [c_int; 5usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDesc {
    pub d_descriptor: c_int,
    pub d_id: c_ulonglong,
}

pub type DoorFunc = unsafe extern "C" fn(
    *mut c_void,
    *mut c_char,
    usize,
    *mut DoorDesc,
    c_uint,
);

extern "C" {
    pub fn door_create(
        server_procedure: DoorFunc,
        cookie: *mut c_void,
        attributes: c_uint,
    ) -> c_int;
    pub fn door_call(d: c_int, params: *mut DoorArg) -> c_int;
    pub fn fattach(fildes: c_int, path: *const c_char) -> c_int;
    pub fn door_return(
        data_ptr: *mut c_char,
        data_size: usize,
        desc_ptr: *mut DoorDesc,
        num_desc: c_uint,
    ) -> c_int;
}
