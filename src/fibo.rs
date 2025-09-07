#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use std::time::Instant;
use std::sync::Mutex;

static total_time: Mutex<u128> = Mutex::new(0);

pub fn fibonacci_recursive(n: u32) -> u64 {
    if n <= 1 { return n as u64; }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

pub fn fibonacci_iterative(n: u32) -> u64 {
    if n <= 1 { return n as u64; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let t = a + b;
        a = b;
        b = t;
    }
    b
}