mod under1000;
mod under100000;

pub use under1000::UNDER_1000;
pub use under100000::UNDER_100000;

pub fn is_prime(n: u32) -> bool {
    !(n < 2
        || UNDER_100000.into_iter().any(|p| n % p == 0 && n != p)
        || ((UNDER_100000[UNDER_100000.len() - 1] + 2)..n)
            .step_by(2)
            .any(|d| n % d == 0))
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
