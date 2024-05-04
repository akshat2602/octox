extern crate alloc;

use crate::println;
use crate::ssht::CONCURRENTHASHMAP;
use crate::trap::TICKS;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn bench_start() {
    // Record the start time of the benchmark without the std library
    let start = *TICKS.lock();

    let mut thread_time = Vec::new();

    // Insert and get values concurrently
    for i in 0..2000 {
        let start = *TICKS.lock();
        CONCURRENTHASHMAP.lock().insert(i, i * 2);
        let _ = CONCURRENTHASHMAP.lock().get(&i);
        // Optionally check or use the retrieved value
        let end = *TICKS.lock();
        thread_time.push(end - start);
    }

    // Record the end time of the benchmark without the std library
    let end = *TICKS.lock();

    // Print the elapsed time
    println!("Elapsed time for the benchmark: {} ticks", end - start);

    // Time taken is in ticks, to convert it to seconds, divide by the frequency of the timer
    // Frequency of the timer is 10000000 Hz as found in https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
    let frequency = 10000000;
    let elapsed_time = (end - start) as f64 / frequency as f64;
    println!("Elapsed time for the benchmark: {} seconds", elapsed_time);
}
