#![no_std]

use alloc::{string::{String, ToString}, vec::Vec,vec};
use ulib::{env, print, println, process::{Child, Command}, sys};
extern crate alloc;

static NUM_PROCESSES: i32 = 10;

fn main() {
    // let _ = accessbench().unwrap();
    // TODO: Change these variables to be inferred from the command line arguments.
    // let locktype = "ticket";
    // let numthreads = 4;
    // let contention = 0;
    let args = env::args().collect::<Vec<&str>>();
    println!("bench {} is {}", args[0], args[1]);
    if args[1] == "start" {
        let start_all = sys::uptime().unwrap();
        start(args[2]);
        println!{"Time to complete all: {}",sys::uptime().unwrap() - start_all};
    } else if args[1] == "run" { 
        accessbench(args[2], args[3]);
    }
    
}

fn start(overall_benchmark_idea: &str) {
    println!("started");
    let mut children: Vec<Option<Child>> = vec![];
    let mut previous_command: Option<Child> = None;

    let bench_strategies = get_strategies(overall_benchmark_idea);
    
    for i in 0..NUM_PROCESSES {
        match Command::new("accessbench")
        .args(vec!["run",&i.to_string(),bench_strategies[i as usize]])
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
        // children[children.len()-1];
        // if let Some(mut unwrappedChild) = previous_command {
        //     unwrappedChild.wait();
        // }
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
    sys::accessbench(-1,1);
}

// fn accessbench() -> sys::Result<usize> {
fn accessbench(pno_str: &str, bench_strategy_str: &str) {

    sys::sleep(100);

    let pno: i32 = pno_str.parse().unwrap();
    let bench_strategy: i32 = bench_strategy_str.parse().unwrap();

    println!("{} start: {}",pno,sys::uptime().unwrap());
    // sys::sleep(100);
    println!("{} running",pno);
    sys::accessbench(pno, bench_strategy);
    println!("{} end: {}",pno,sys::uptime().unwrap());
    // sys::accessbench(0)
}

fn get_strategies(overall_benchmark_idea: &str) -> Vec<&str> {
    let mut strats = Vec::new();
    for i in 0..NUM_PROCESSES {
        strats.push("1"); //1 is 4000000 accesses
    }
    strats
}