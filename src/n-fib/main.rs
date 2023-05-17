use std::io;

fn fib(n: u32) -> u32 {
    if n == 0 {
        panic!("0 is incorrect number.")
    }

    let mut prev_2 = 0;
    let mut prev_1 = 1;
    let mut val = 0;

    if n <= 2 {
        return n - 1;
    }

    for _ in 2..n {
        val = prev_2 + prev_1;
        prev_2 = prev_1;
        prev_1 = val;
    }

    val
}

fn main() {
    println!("Enter fibonachi sequence number");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read a line.");

    let n: u32 = n.trim().parse().expect("Not a number");

    let fib_val = fib(n);
    println!("{n} value of fibonachi sequence is {fib_val}");
}
