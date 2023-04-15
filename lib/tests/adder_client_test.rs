// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

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
fn test_add_client() {
    let file = File::open("/tmp/addr-test-door").expect("open fd");

    let x: u64 = 74;
    let res: Wrapped = door_call(file.as_raw_fd(), x);

    assert_eq!(res.val, 121);
    assert_eq!(res.other, 99);
}
