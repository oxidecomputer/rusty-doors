// Copyright 2021 Oxide Computer Company

use std::ffi::CString;
use rusty_doors::{door_run, door_create};
use rusty_doors_macros::door;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: u64,
    other: u16,
}

#[door]
fn serv_proc(x: u64) -> Wrapped {
    let res = x + 47;
    println!("ARG: {}", x);
    return Wrapped{val: res, other: 99};
}

#[test]
fn test_add_server() {
    let path = CString::new("/tmp/addr-test-door").expect("cstring");
    let fd = door_create(serv_proc);
    door_run(fd, path.as_c_str());
}