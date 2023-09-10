use play_rs::primes;

const MULTIPLIER: f32 = 1024.0;

fn main() {
    let mut sieve = primes::Sieve::default();
    let mut triples: Vec<_> = (2..40u32)
        .map(move |n| {
            let d: u32 = sieve.factors(n).map(|(_prime, power)| power).sum();
            let r = (n as f32 * MULTIPLIER / d as f32).round() as u32;
            (r, n, d)
        })
        .collect();
    triples.sort();
    for (i, (r, n, d)) in triples.into_iter().enumerate() {
        let rank = i + 1;
        println!("{rank:2}: {n:2} / {d:2} = {:5.2}", r as f32 / MULTIPLIER);
    }
}
