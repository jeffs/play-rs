use play_rs::fizz_buzz::fizz_buzz;

const DEFAULT_N: usize = 100;

fn parse_args() -> Result<usize, &'static str> {
    let mut args = std::env::args().skip(1);
    let Some(arg) = args.next() else {
        return Ok(DEFAULT_N);
    };
    let Ok(n) = arg.parse() else {
        return Err("bad count");
    };
    if args.next().is_some() {
        return Err("too many args");
    }
    Ok(n)
}

fn main() {
    let n = parse_args().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        eprintln!("Usage: fizz-buzz [count]");
        std::process::exit(2);
    });
    for (i, s) in (1..).zip(fizz_buzz()).take(n) {
        println!("{i:>4}    {s}");
    }
}
