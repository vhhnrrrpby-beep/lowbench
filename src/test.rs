use super::*;
extern crate std;

use core::time::Duration;
use std::println;
use std::time::Instant;

#[cfg(not(feature = "std"))]
impl Timer for Instant {
    fn now() -> Self {
        Instant::now()
    }
    fn elapsed(&self) -> Duration {
        Instant::elapsed(self)
    }
}

fn add(a: usize, b: i32) -> i32 {
    (a as i32) + b
}

fn fib() {
    let _ = 1 + 1;
}

#[test]
#[cfg(feature = "std")]
fn bench_test() {
    println!("first test \n");
    bench(|| {
        for (i, x) in (3..104).enumerate() {
            add(i, x);
        }
    });

    println!("second test \n");
    bench(|| {
        fib();
    });

    println!("third test \n");
    bench(|| {});
}

#[test]
#[cfg(not(feature = "std"))]
fn bench_test() {
    println!("first test \n");
    println!(
        "{:?}",
        bench::<Instant>(|| {
            for (i, x) in (3..104).enumerate() {
                add(i, x);
            }
        })
    );

    println!("second test \n");
    println!(
        "{:?}",
        bench::<Instant>(|| {
            fib();
        })
    );

    println!("third test \n");
    println!("{:?}", bench::<Instant>(|| {}));
}
