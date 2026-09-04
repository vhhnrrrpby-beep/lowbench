use std::time::Instant;
use std::hint::black_box;
use std::time::Duration;

#[cfg(test)]
mod test;


pub fn bench(mut f:impl FnMut()) {
    println!("Benchmark Running...");
    for _ in 0..1000 {
        f();
    };

    let mut results:[Duration;3] = [Duration::new(0,0);3];


    for i in 0..3 {
        let start = Instant::now();
        for _ in 0..1000 {
            black_box(f());
        };
        let elapsed = start.elapsed();
        results[i] = elapsed;
    }

    println!("Benchmark Result");
    println!("averages :: {:?}",results.map(|x| x/1000));
}