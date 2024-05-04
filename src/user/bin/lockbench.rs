#![no_std]
use core::usize;

use ulib::sys;
extern crate alloc;

fn main() {
    let _ = lockbench().unwrap();
    // TODO: Change these variables to be inferred from the command line arguments.
    // let locktype = "ticket";
    // let numthreads = 4;
    // let contention = 0;
}

fn lockbench() -> sys::Result<usize> {
    sys::createbench()
}
