use play_rs::primes::primes;

fn main() {
    let target = 2147483647;
    match primes()
        .take_while(|&n| n <= target)
        .enumerate()
        .inspect(|(i, n)| {
            if i % 1000 == 0 {
                eprintln!("{i:8} {n}");
            }
        })
        .last()
    {
        Some((index, value)) if value == target => println!("{target} found at index {index}"),
        _ => println!("{target} is not prime"),
    }
}
