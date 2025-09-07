#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]


use factorial::Factorial;

pub fn pi_calculator(n : u128) -> f64 {

    let mut pi = 0.0;
    let mut sum = 0.0;
    for k in 0..n {
        sum += Factorial::factorial(4*k)/Factorial::factorial(k).pow(4) as u128 * (1103 + 26390*k) as f64 / (396.0_f64).powi((4*k) as i32); // (4k)!/(k!)^4 * (1103 + 26390k) / 396^(4k) 
    }
    pi = 2*2.powi(1/2)/(99*99) * sum; // 1/pi = 2*sqrt(2)/(99^2) * sum 
    pi = 1.0/pi;
    pi
}