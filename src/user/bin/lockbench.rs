#![no_std]
use core::usize;

use alloc::vec::Vec;
use ulib::{
    env,sys,print,println
};
extern crate alloc;

fn main() {
    let args = env::args().collect::<Vec<&str>>();
    println!("bench {} is {}", args[0], args[1]);
    let res = lockbench(args[1],args[2]).unwrap();
}

fn lockbench(ig: &str, benchno: &str) -> sys::Result<usize>{
    if ig == "i" {
        sys::createbench(benchno.parse().unwrap())
    } else {
        sys::accessbench(benchno.parse().unwrap(), 1)
    }
}