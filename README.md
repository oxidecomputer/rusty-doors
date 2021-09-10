# Rusty Doors

A streamlined and safe interface for creating and using Unix Doors.

## A simple door server

```rust
use std::fs::File;
use rusty_doors::{door_call, sys::ulong_t};
use std::os::unix::io::AsRawFd;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: ulong_t,
    other: u16,
}

// To make a door handler just apply the #[door] attribute macro
#[door]
fn serv_proc(x: ulong_t) -> Wrapped {
    let res = x + 47;
    return Wrapped{val: res, other: 99};
}


fn main() {
    // create and run the door server
    let path = CString::new("/tmp/addr-test-door").expect("cstring");
    let fd = door_create(serv_proc);
    door_run(fd, path.as_c_str());
}
```

## A door client

```rust
use std::ffi::CString;
use rusty_doors::{door_run, door_create, sys::ulong_t};
use rusty_doors_macros::door;

fn main() {

    let file = File::open("/tmp/addr-test-door").expect("open fd");

    let x: ulong_t = 74;
    let res: Wrapped = door_call(file.as_raw_fd(), x);

    assert_eq!(res.val, 121);
    assert_eq!(res.other, 99);
}
```
