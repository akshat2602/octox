extern crate alloc;

use crate::println;
use crate::ssht::CONCURRENTHASHMAP;
use crate::trap::TICKS;

pub fn bench_start(pno: i32, bench_strategy: i32) {
    // Record the start time of the benchmark without the std library
    let start = *TICKS.lock();
    
    match bench_strategy {
        1 => bench_1(),
        _ => ()
    }

    let end = *TICKS.lock();

    // Print the elapsed time
    unsafe{
        println!("Processor: {}, Size: {} , Elapsed time for the benchmark: {} ticks", pno, CONCURRENTHASHMAP.size(),end - start);
    }

    // Time taken is in ticks, to convert it to seconds, divide by the frequency of the timer
    // Frequency of the timer is 10000000 Hz as found in https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
    let frequency = 10000000;
    let elapsed_time = (end - start) as f64 / frequency as f64;
    println!("Elapsed time for the benchmark: {} seconds", elapsed_time);
}


fn bench_1() {
    // Insert and get values concurrently
    for i in 0..4000000 {
        // let start = *TICKS.lock();
        unsafe {
            CONCURRENTHASHMAP.insert(i, i * 2);
            CONCURRENTHASHMAP.get(&i).unwrap_or_else(|| {
                &0
            });
        }
        // Optionally check or use the retrieved value
        // let end = *TICKS.lock();
        // thread_time.push(end - start);
    }
}
