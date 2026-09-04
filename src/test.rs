use super::*;

fn add(a:usize,b:i32) -> i32 {
    (a as i32)+b
}

fn fib() { 
    let _ = 1+1;
}



#[test] 
fn bench_test() {
    println!("first test \n");
    bench(|| {
        for (i,x) in (3..104).enumerate() {
            add(i,x);
        }
    });

    println!("second test \n");
    bench(|| {
        fib();
    });

    println!("third test \n");
    bench(|| {});
}
