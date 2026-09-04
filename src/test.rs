use std::hint::black_box;
use super::*;

fn add(a:usize,b:i32) -> i32 {
    (a as i32)+b
}

fn fib() { 
    black_box(1+1);
}

#[test] 
fn bench_test() {
    println!("first test \n\n\n");
    bench(|| {
        for (i,x) in (3..104).enumerate() {
            add(i,x);
        }
    });

    println!("second test \n\n\n");
    bench(|| {
        fib();
    });
}
