use std::{env, num::NonZeroUsize, process::exit};

fn main() {
    // Parse N.
    let (Some(n), None) = ({
        let mut args = env::args().skip(1);
        (
            args.next().and_then(|arg| arg.parse::<NonZeroUsize>().ok()),
            args.next(),
        )
    }) else {
        eprintln!("usage: fibloop N");
        exit(2)
    };

    // Print first N Fibonacci numbers.
    print!("1");
    let (mut a, mut b) = (1, 2);
    for _ in 1..n.get() {
        print!(" {a}");
        (a, b) = (b, a + b);
    }
    println!();
}
