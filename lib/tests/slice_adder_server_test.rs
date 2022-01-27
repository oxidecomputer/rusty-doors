// Copyright 2021 Oxide Computer Company

use rusty_doors::{door_create, door_run};
use rusty_doors_macros::door;
use std::ffi::CString;
use std::thread::{sleep, spawn};
use std::time::Duration;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: u64,
    other: u16,
}

#[door]
fn serv_proc(x: [u64; 4]) -> Wrapped {
    let res = x[0] + x[1] + x[2] + x[3];
    println!("ARG: {:?}", x);
    return Wrapped {
        val: res,
        other: 1701,
    };
}

#[test]
fn test_add_slice_server() {
    spawn(|| {
        let path = CString::new("/tmp/addr-test-door-slice").expect("cstring");
        let fd = door_create(serv_proc);
        door_run(fd, path.as_c_str());
    });
    sleep(Duration::from_secs(5));
}
