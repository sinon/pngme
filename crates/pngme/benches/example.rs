use pngme_lib::encode;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
