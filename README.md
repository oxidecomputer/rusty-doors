# Rusty Doors

A streamlined and safe interface for creating and using Unix Doors.

## A simple door server

```rust
use rusty_doors::{door_create, door_run};
use rusty_doors_macros::door;
use std::ffi::CString;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: u64,
    other: u16,
}

// To make a door handler just apply the #[door] attribute macro
#[door]
fn serv_proc(x: u64) -> Wrapped {
    let res = x + 47;
    println!("ARG: {}", x);
    return Wrapped {
        val: res,
        other: 99,
    };
}

fn main() {
    let path = CString::new("/tmp/addr-test-door").expect("cstring");
    let fd = door_create(serv_proc);
    door_run(fd, path.as_c_str());
}
```

## A door client

```rust
use rusty_doors::door_call;
use std::fs::File;
use std::os::unix::io::AsRawFd;

#[derive(Default)]
#[repr(C)]
struct Wrapped {
    val: u64,
    other: u16,
}

fn main() {
    let file = File::open("/tmp/addr-test-door").expect("open fd");

    let x: u64 = 74;
    let res: Wrapped = door_call(file.as_raw_fd(), x);

    assert_eq!(res.val, 121);
    assert_eq!(res.other, 99);
}
```
