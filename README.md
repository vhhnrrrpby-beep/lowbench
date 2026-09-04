# lowbench
This crate is a tiny benchmark.<br>
lowbench has only one function::bench.<br>
I have almost no plans to update this.
## usage
```rust
use lowbench::bench;

fn main() {
    bench(|| {
        Your_function();
    });
}

//output

//Benchmark Running...
//
//Benchmark Result
//average:: [10ns, 11ns, 12ns]
//attempts:: 10000
//Running time :: 10ms
```
