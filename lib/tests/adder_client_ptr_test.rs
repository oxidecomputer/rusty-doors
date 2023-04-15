// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use rusty_doors::door_callp;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::ptr;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: u64,
    other: u16,
}

#[test]
fn test_add_client_ptr() {
    let file = File::open("/tmp/addr-test-door").expect("open fd");

    unsafe {
        let x: u64 = 47;
        let mut res: *mut Wrapped = ptr::null_mut();
        let out: *mut Wrapped = door_callp(
            file.as_raw_fd(),
            x,
            ptr::NonNull::new(&mut res).unwrap(),
        );

        assert_eq!((*res).val, 94);
        assert_eq!((*res).other, 99);
        assert_eq!((*out).val, 94);
        assert_eq!((*out).other, 99);
        assert_eq!(out, res);
    }
}
