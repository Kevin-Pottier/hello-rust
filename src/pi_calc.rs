#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]


pub fn pi_calculator(n : u128) -> f64 {

    let mut pi = 0.0;
    let mut term : f64 = 0.0;
    let mut sum = 0.0;
    for k in 0..n {
        term = factorial(4*k) as f64 / power(factorial(k) as f64, 4) as f64 * (1103 + 26390*k) as f64 / power(396.0_f64, 4*k ) as f64; // (4k)!/(k!)^4 * (1103 + 26390k) / 396^(4k)
        println!("term[{}] = {:.15}", k, term);
        sum += term;
    }
    println!("sum = {:.15}", sum);
    pi = 2.0 * power(2.0, 1/2)/ power(99.0, 2) * sum; // 1/pi = 2*sqrt(2)/(99^2) * sum
    print!("1/pi = {:.15}\n", pi);
    pi = 1.0/pi;
    pi
}

pub fn factorial(n: u128) -> u128 {
    if n == 0 {
        return 1;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

pub fn power(base: f64, exp: u128) -> f64 {
    let mut result = 1.0;
    for _ in 0..exp {
        result *= base;
    }
    result
}
