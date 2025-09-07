#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use std::time::Instant;
use std::sync::Mutex;

// Matrice 2x2
#[derive(Clone, Copy, Debug)]
struct Mat2 {
    a: u128, b: u128,
    c: u128, d: u128,
}

impl Mat2 {
    #[inline]
    const fn id() -> Self { Self { a: 1, b: 0, c: 0, d: 1 } }

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }
}

/// Exponentiation rapide: base^exp
fn mat_pow(mut base: Mat2, mut exp: u128) -> Mat2 {
    let mut res = Mat2::id();
    while exp > 0 {
        if exp & 1 == 1 {
            res = res.mul(base);
        }
        base = base.mul(base);
        exp >>= 1;
    }
    res
}

/// Fibonacci via matrices:
/// [[F_{n+1}, F_n], [F_n, F_{n-1}]] = [[1,1],[1,0]]^n
pub fn fib_matrix(n: u128) -> u128 {
    // valable pour n = 0 aussi (donne 0)
    let fib_base = Mat2 { a: 1, b: 1, c: 1, d: 0 };
    mat_pow(fib_base, n).b
}




pub fn fibonacci_recursive(n: u128) -> u128 {
    if n <= 1 { return n; }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

pub fn fib_memo(n: usize) -> u128 {
    fn rec(k: usize, memo: &mut Vec<Option<u128>>) -> u128 {
        if k <= 1 { return k as u128; }
        if let Some(v) = memo[k] { return v; }
        let v = rec(k - 1, memo) + rec(k - 2, memo);
        memo[k] = Some(v);
        v
    }
    let mut memo = vec![None; n + 1];
    rec(n, &mut memo)
}

pub fn fib_fast_doubling(n: u128) -> u128 {
    fn fd(k: u128) -> (u128, u128) { // retourne (F(k), F(k+1))
        if k == 0 { return (0, 1); }
        let (a, b) = fd(k >> 1);          // a = F(k/2), b = F(k/2+1)
        let c = a * (b * 2 - a);          // F(2m)
        let d = a*a + b*b;                // F(2m+1)
        if k & 1 == 0 { (c, d) } else { (d, c + d) }
    }
    fd(n).0
}


pub fn fibonacci_iterative(n: u128) -> u128 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let t = a + b;
        a = b;
        b = t;
    }
    b
}