# lowbench
This crate is a tiny benchmark.
lowbench has only one function.
## usage
```rust
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
```
## implementation
```rust
fn bench(mut f:impl FnMut()) ;
```