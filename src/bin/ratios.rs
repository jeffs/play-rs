use play_rs::primes;

/// Used to convert f32 ratios to and from u32 so we can sort them without
/// worrying about floating-point wackiness (NaN, Inf, etc.).
const MULTIPLIER: f32 = 1024.0;

fn print(scored: &[(i32, usize, u32, u32, u32)]) {
    for &(score, rank, r, n, d) in scored {
        println!(
            "{rank:2}: {n:2} / {d:2} = {:5.2} {score:8}",
            r as f32 / MULTIPLIER
        );
    }
}

fn main() {
    // Rank numbers by ratio of their value to how many prime factors they have.
    let mut sieve = primes::Sieve::default();
    let mut triples: Vec<_> = (2..40u32)
        .map(move |n| {
            let d: u32 = sieve.factors(n).map(|(_prime, power)| power).sum();
            let r = (n as f32 * MULTIPLIER / d as f32).round() as u32;
            (r, n, d) // ratio, numerator, denominator
        })
        .collect();
    triples.sort();

    // Score numbers according to their value minus their rank.
    let mut scored: Vec<_> = triples
        .into_iter()
        .enumerate()
        .map(|(i, (r, n, d))| {
            let rank = i + 1;
            let score = n as i32 - rank as i32;
            (score, rank, r, n, d)
        })
        .collect();

    println!("# Fewest digits per factor");
    print(&scored);

    scored.sort();
    scored.reverse();
    println!("\n# Best rank relative to value");
    print(&scored);
}
