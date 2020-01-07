use ez_trace_macros::ez_trace;

fn main() {
    println!("Starting");
    fibonacci(5);
    println!("Finished");
}

#[ez_trace(start = "{name}({args})", end = "{name}({args}) -> {result}")]
fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
