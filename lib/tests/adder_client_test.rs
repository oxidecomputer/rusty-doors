// Copyright 2021 Oxide Computer Company

use std::fs::File;
use rusty_doors::{door_call, sys::ulong_t};
use std::os::unix::io::AsRawFd;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: ulong_t,
    other: u16,
}

#[test]
fn test_add_client() {

    let file = File::open("/tmp/addr-test-door").expect("open fd");

    let x: ulong_t = 74;
    let res: Wrapped = door_call(file.as_raw_fd(), x);

    assert_eq!(res.val, 121);
    assert_eq!(res.other, 99);
}

