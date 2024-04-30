#![no_std]
use core::usize;

use ulib::{
    env,sys,print,println
};
extern crate alloc;

fn main() {
    let args = env::args();
    for arg in args.skip(1) {
        let res = lockbench(arg).unwrap();
        println!("Result of bench {} is {}", arg, res);
    }
}

fn lockbench(benchno: &str) -> sys::Result<usize>{
    sys::bench(benchno.parse().unwrap())
}
