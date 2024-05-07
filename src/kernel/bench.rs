extern crate alloc;

use crate::println;
use crate::ssht_spinlock::CONCURRENTHASHMAPSPINLOCK;
use crate::ssht_spinlock_faa::CONCURRENTHASHMAPSPINLOCKFAA;
use crate::ssht_spinlock_tas::CONCURRENTHASHMAPSPINLOCKTAS;
use crate::ssht_ticket::CONCURRENTHASHMAP;
use crate::ssht_sleep::CONCURRENTHASHMAPSLEEPLOCK;
use crate::trap::TICKS;

pub fn bench_start(pno: i32, bench_strategy: i32, contention: i32) {
    // Record the start time of the benchmark without the std library
    let start = *TICKS.lock();

    let (result, avg_lat) = match bench_strategy {
        1 => bench_ticket(contention, pno),
        2 => bench_spin(contention, pno),
        3 => bench_spin_faa(contention, pno),
        4 => bench_spin_tas(contention, pno),
        5 => bench_sleep(contention, pno),
        _ => default_value(),
    };

    let end = *TICKS.lock();

    // Print the elapsed time for the benchmark
    println!(
        "Processor: {}, Size: {} , Elapsed time for the benchmark: {} ticks, Average latency: {} ticks",
        pno,
        result,
        end - start,
        avg_lat
    );

    // Time taken is in ticks, to convert it to seconds, divide by the frequency of the timer
    // Frequency of the timer is 10000000 Hz as found in https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
    let frequency = 10000000;
    let elapsed_time = (end - start) as f64 / frequency as f64;
    println!("Elapsed time for the benchmark: {} seconds", elapsed_time);
}

fn default_value() -> (i32, f32) {
    (-1, -1 as f32)
}

fn bench_ticket(contention: i32, proc_num: i32) -> (i32, f32) {
    // Insert and get values concurrently
    let mut avg_time: i32 = 0;
    for i in 0..4000000 {
        let start = *TICKS.lock();
        if contention == 1 {
            unsafe {
                // Highest contention
                CONCURRENTHASHMAP.insert(i, i * 2);
                CONCURRENTHASHMAP.get(&i).unwrap_or_else(|| &0);
            }
        } else if contention == 2 {
            unsafe {
                // Lowest contention
                CONCURRENTHASHMAP.insert(proc_num % 5, i * 2);
                CONCURRENTHASHMAP
                    .get(&(proc_num % 5))
                    .unwrap_or_else(|| &0);
            }
        } else if contention == 3 {
            unsafe {
                // No contention
                CONCURRENTHASHMAP.insert(proc_num % 10, i * 2);
                CONCURRENTHASHMAP
                    .get(&(proc_num % 10))
                    .unwrap_or_else(|| &0);
            }
        }
        // Optionally check or use the retrieved value
        let end = *TICKS.lock();
        // thread_time.push(end - start);
        avg_time += (end - start) as i32;
    }
    return (
        unsafe { CONCURRENTHASHMAP.size() as i32 },
        (avg_time as f32)/(4000000 as f32),
    );
}

fn bench_spin(contention: i32, proc_num: i32) -> (i32, f32) {
    // Insert and get values concurrently
    let mut avg_time: i32 = 0;
    for i in 0..4000000 {
        let start = *TICKS.lock();
        if contention == 1 {
            unsafe {
                CONCURRENTHASHMAPSPINLOCK.insert(1, i * 2);
                CONCURRENTHASHMAPSPINLOCK.get(&1).unwrap_or_else(|| &0);
            }
        } else if contention == 2 {
            unsafe {
                // Lowest contention
                CONCURRENTHASHMAPSPINLOCK.insert(proc_num % 5, i * 2);
                CONCURRENTHASHMAPSPINLOCK
                    .get(&(proc_num % 5))
                    .unwrap_or_else(|| &0);
            }
        } else if contention == 3 {
            unsafe {
                CONCURRENTHASHMAPSPINLOCK.insert(proc_num % 10, i * 2);
                CONCURRENTHASHMAPSPINLOCK
                    .get(&(proc_num % 10))
                    .unwrap_or_else(|| &0);
            }
        }
        // Optionally check or use the retrieved value
        let end = *TICKS.lock();
        // thread_time.push(end - start);
        avg_time += (end - start) as i32;
    }
    return (
        unsafe { CONCURRENTHASHMAPSPINLOCK.size() as i32 },
        (avg_time as f32)/(4000000 as f32),
    );
}

fn bench_spin_faa(contention: i32, proc_num: i32) -> (i32, f32) {
    // Insert and get values concurrently
    let mut avg_time: i32 = 0;
    for i in 0..4000000 {
        let start = *TICKS.lock();
        if contention == 1 {
            unsafe {
                CONCURRENTHASHMAPSPINLOCKFAA.insert(i, i * 2);
                CONCURRENTHASHMAPSPINLOCKFAA.get(&i).unwrap_or_else(|| &0);
            }
        } else if contention == 2 {
            unsafe {
                // Lowest contention
                CONCURRENTHASHMAPSPINLOCKFAA.insert(proc_num % 5, i * 2);
                CONCURRENTHASHMAPSPINLOCKFAA
                    .get(&(proc_num % 5))
                    .unwrap_or_else(|| &0);
            }
        } else if contention == 3 {
            unsafe {
                CONCURRENTHASHMAPSPINLOCKFAA.insert(proc_num % 10, i * 2);
                CONCURRENTHASHMAPSPINLOCKFAA
                    .get(&(proc_num % 10))
                    .unwrap_or_else(|| &0);
            }
        }
        // Optionally check or use the retrieved value
        let end = *TICKS.lock();
        // thread_time.push(end - start);
        avg_time += (end - start) as i32;
    }
    return (
        unsafe { CONCURRENTHASHMAPSPINLOCKFAA.size() as i32 },
        (avg_time as f32)/(4000000 as f32),
    );
}

fn bench_spin_tas(contention: i32, proc_num: i32) -> (i32, f32) {
    let mut avg_time: i32 = 0;
    // Insert and get values concurrently
    for i in 0..4000000 {
        let start = *TICKS.lock();
        if contention == 1 {
            unsafe {
                CONCURRENTHASHMAPSPINLOCKTAS.insert(i, i * 2);
                CONCURRENTHASHMAPSPINLOCKTAS.get(&i).unwrap_or_else(|| &0);
            }
        } else if contention == 2 {
            unsafe {
                // Lowest contention
                CONCURRENTHASHMAPSPINLOCKTAS.insert(proc_num % 5, i * 2);
                CONCURRENTHASHMAPSPINLOCKTAS
                    .get(&(proc_num % 5))
                    .unwrap_or_else(|| &0);
            }
        } else if contention == 3 {
            unsafe {
                CONCURRENTHASHMAPSPINLOCKTAS.insert(proc_num % 10, i * 2);
                CONCURRENTHASHMAPSPINLOCKTAS
                    .get(&(proc_num % 10))
                    .unwrap_or_else(|| &0);
            }
        }
        // Optionally check or use the retrieved value
        let end = *TICKS.lock();
        // thread_time.push(end - start);
        avg_time += (end - start) as i32;
    }
    return (
        unsafe { CONCURRENTHASHMAPSPINLOCKTAS.size() as i32 },
        (avg_time as f32)/(4000000 as f32),
    );
}


fn bench_sleep(contention: i32, proc_num: i32) -> (i32, i32) {
    let mut avg_time: i32 = 0;
    // Insert and get values concurrently
    for i in 0..4000000 {
        let start = *TICKS.lock();
        if contention == 1 {
            unsafe {
                CONCURRENTHASHMAPSLEEPLOCK.insert(i, i * 2);
                CONCURRENTHASHMAPSLEEPLOCK.get(&i).unwrap_or_else(|| &0);
            }
        } else if contention == 2 {
            unsafe {
                // Lowest contention
                CONCURRENTHASHMAPSLEEPLOCK.insert(proc_num % 12, i * 2);
                CONCURRENTHASHMAPSLEEPLOCK
                    .get(&(proc_num % 12))
                    .unwrap_or_else(|| &0);
            }
        } else if contention == 3 {
            unsafe {
                CONCURRENTHASHMAPSLEEPLOCK.insert(proc_num % 25, i * 2);
                CONCURRENTHASHMAPSLEEPLOCK
                    .get(&(proc_num % 25))
                    .unwrap_or_else(|| &0);
            }
        }
        // Optionally check or use the retrieved value
        let end = *TICKS.lock();
        // thread_time.push(end - start);
        avg_time += (end - start) as i32;
    }
    return (
        unsafe { CONCURRENTHASHMAPSLEEPLOCK.size() as i32 },
        avg_time / 4000000,
    );
}
