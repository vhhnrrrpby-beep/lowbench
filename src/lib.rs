use std::time::Instant;
use std::hint::black_box;
use std::time::Duration;

#[cfg(test)]
mod test;

/// Runs a benchmark on the given function.
///
/// The benchmark performs a warm-up phase and then
/// measures the function three times.

const MAX_WARMUP_ATTEMPT:u8 = 10;
const WARMUP_COUNTS: [usize; 3] = [100, 1000, 10000];
pub fn bench(mut f:impl FnMut())  -> [Duration;3]{
    let all_start = Instant::now();
    println!("Benchmark Running...");

    let mut warmup_count = 0;
    let mut tests:[Duration;2] = [Duration::new(0,0),Duration::new(0,0)];
    for i in 0..2 {
        let start = Instant::now();
        for _ in 0..(WARMUP_COUNTS[warmup_count]) {
            black_box(f());
        }
        let elapsed = start.elapsed();
        tests[i] = elapsed/(WARMUP_COUNTS[warmup_count] as u32);
        if elapsed.as_nanos() < Duration::from_micros(10).as_nanos() {
            warmup_count += 1;
        };
    }

    let mut warmup_attempt = 0;
    while tests[0].as_nanos().abs_diff(tests[1].as_nanos()) > Duration::new(0,100).as_nanos()
    {
        let start = Instant::now();
        for _ in 0..(WARMUP_COUNTS[warmup_count]) {
            black_box(f());
        }
        let elapsed = start.elapsed();
        tests[0] = tests[1];
        tests[1] = elapsed/(WARMUP_COUNTS[warmup_count] as u32);
        warmup_attempt += 1 ;
        if warmup_attempt > MAX_WARMUP_ATTEMPT {
            break;
        }
    }
    
    let count = set_count(tests[1]);

    let mut results:[Duration;3] = [Duration::new(0,0);3];

    for i in 0..3 {
        let start = Instant::now();
        for _ in 0..count {
            black_box(f());
        };
        let elapsed = start.elapsed();
        results[i] = elapsed;
    }

    println!("Benchmark Result");
    println!("averages :: {:?}",results.map(|x| x/(count as u32)));
    println!("attempts :: {count}");
    println!("Running time :: {:?}",all_start.elapsed());
    results.map(|x| x/(count as u32))
}

const MAX_ATTEMPT:usize = 100_000_000;
const MIN_ATTEMPT:usize = 100;
fn set_count(tests:Duration) -> usize {
    if tests.as_nanos() == 0 {
        return MAX_ATTEMPT;
    }
    let count = (Duration::from_millis(10).as_nanos() / tests.as_nanos()) as usize;

    if count < MIN_ATTEMPT {
        MIN_ATTEMPT
    } else if count > MAX_ATTEMPT {
        MAX_ATTEMPT
    } else {count}
}