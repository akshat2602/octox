#![no_std]

use alloc::{string::{String, ToString}, vec::Vec,vec};
use ulib::{env, print, println, process::{Child, Command}, sys};
extern crate alloc;

fn main() {
    // let _ = accessbench().unwrap();
    // TODO: Change these variables to be inferred from the command line arguments.
    // let locktype = "ticket";
    // let numthreads = 4;
    // let contention = 0;
    let args = env::args().collect::<Vec<&str>>();
    println!("bench {} is {}", args[0], args[1]);
    if args[1] == "start" {
        start();
    } else if args[1] == "run" { 
        accessbench(args[2]);
    }
    
}

fn start() {
    println!("started");
    let mut children: Vec<Option<Child>> = vec![];
    let mut previous_command: Option<Child> = None;
    
    for i in 0..10 {
        match Command::new("accessbench")
        .args(vec!["run",&i.to_string()])
            // .stdin(stdin)
            // .stdout(stdout)
            .spawn()
        {
            Ok(child) => previous_command = Some(child),
            Err(e) => {
                previous_command = None;
                println!("{}", e);
            }
        }

        children.push(previous_command);
    }

    println!("All processes launched!");
    for child in children {
        if let Some(mut unwrappedChild) = child {
            unwrappedChild.wait();
        }
        else {

        }
    }
    println!("All processes done");
}

// fn accessbench() -> sys::Result<usize> {
fn accessbench(pno: &str) {
    println!("{} start: {}",pno,sys::uptime().unwrap());
    // sys::sleep(100);
    println!("{} running",pno);
    sys::accessbench(0);
    println!("{} end: {}",pno,sys::uptime().unwrap());
    // sys::accessbench(0)
}
