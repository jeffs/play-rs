pub const UNDER_1000: [u32; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

pub fn is_prime(n: u32) -> bool {
    !(n < 2 || (n % 2 == 0 && n != 2) || (3..n).step_by(2).any(|d| n % d == 0))
}

pub struct Primes(u32);

impl Default for Primes {
    fn default() -> Self {
        Primes(2)
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.0;
        self.0 = if old == 2 {
            3
        } else {
            ((old + 2)..)
                .step_by(2)
                .find(|&n| is_prime(n))
                .expect("There should always be another prime.")
        };
        Some(old)
    }
}

pub fn primes() -> Primes {
    Primes::default()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_is_prime() {
        let known: HashSet<_> = UNDER_1000.into_iter().collect();
        for n in 0..1000 {
            assert_eq!(is_prime(n), known.contains(&n));
        }
    }

    #[test]
    fn test_primes() {
        let want = UNDER_1000;
        let got: Vec<_> = primes().take(want.len()).collect();
        assert_eq!(got, want);
    }
}
