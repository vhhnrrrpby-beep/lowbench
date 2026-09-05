# lowbench
This crate provides a tiny benchmarking utility.<br>
lowbench has only one function::bench.<br>
I have almost no plans to update this.
## Usage
### With std
```toml
[dependencies]
lowbench = "0.2.0" 
```
```rust
use lowbench::bench;

fn main() {
    bench(|| {
        your_function();
    });
}

// output

//Benchmark Running...
//
//Benchmark Result
//average  ::  [10ns, 11ns, 12ns]
//attempts  ::  10000
//Running time  ::  10ms
```
### Without std
```toml
[dependencies]
lowbench = { version = "0.2.0", default-features = false }
``` 
```rust
#![no_std]
use lowbench::{bench,Timer};
use core::time::Duration;

struct YourTimer ;
impl Timer for YourTimer {
    fn now() -> Self {
        // timer implementation
    }

    fn elapsed(&self) -> Duration {
        // timer implementation
    }
}

fn main() {
    let result = bench::<YourTimer>(|| {
        your_function();
    });
}
```