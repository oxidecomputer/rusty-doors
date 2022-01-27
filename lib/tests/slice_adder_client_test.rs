// Copyright 2021 Oxide Computer Company

use rusty_doors::door_call;
use std::fs::File;
use std::os::unix::io::AsRawFd;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: u64,
    other: u16,
}

#[test]
fn test_add_slice_client() {
    let file = File::open("/tmp/addr-test-door-slice").expect("open fd");

    let x: [u64; 4] = [47, 74, 4, 7];
    let res: Wrapped = door_call(file.as_raw_fd(), x);

    assert_eq!(res.val, 132);
    assert_eq!(res.other, 1701);
}
