#![no_std]
use core::usize;

use ulib::sys;
extern crate alloc;

fn main() {
    let _ = accessbench().unwrap();
    // TODO: Change these variables to be inferred from the command line arguments.
    // let locktype = "ticket";
    // let numthreads = 4;
    // let contention = 0;
}

fn accessbench() -> sys::Result<usize> {
    sys::accessbench()
}
