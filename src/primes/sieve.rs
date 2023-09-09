//! TODO: Rename Sieve.  It's not a Sieve of Eratosthenes.

use std::{cell::RefCell, mem};

use super::UNDER_100000;

/// Returns true if none of the known primes divides n (other than n itself).
///
/// Prerequisite: The slice of known primes is sorted.
fn is_prime_known(n: u32, known: &[u32]) -> bool {
    !known
        .into_iter()
        .take_while(|&&p| p < n)
        .any(|p| n % p == 0)
}

pub struct SievePrimes<'a> {
    sieve: &'a Sieve,
    next: u32,
}

impl<'a> Iterator for SievePrimes<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.next == 2 {
            3
        } else {
            let mut next = self.next;
            while !self.sieve.is_prime(next) {
                next += 2;
            }
            next
        };
        Some(mem::replace(&mut self.next, next))
    }
}

#[derive(Default)]
pub struct Sieve {
    known: RefCell<Vec<u32>>,
}

impl Sieve {
    fn extend_past(&self, n: u32) {
        let mut known = self.known.borrow_mut();
        if known.is_empty() {
            known.extend_from_slice(&UNDER_100000);
        }
        let last = known[known.len() - 1];
        if last < n {
            let limit = n * 2; // Arbitrary.
            for candidate in (last..limit).step_by(2) {
                if is_prime_known(candidate, &known) {
                    known.push(candidate);
                }
            }
        }
    }

    pub fn is_prime(&self, n: u32) -> bool {
        self.extend_past(n);
        self.known.borrow().binary_search(&n).is_ok()
    }

    pub fn new(known: &[u32]) -> Sieve {
        Sieve {
            known: RefCell::new(known.to_vec()),
        }
    }

    pub fn primes(&self) -> SievePrimes {
        SievePrimes {
            sieve: self,
            next: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primes::UNDER_1000;

    use super::*;

    #[test]
    fn test_sieve_is_prime() {
        let sieve = Sieve::new(&UNDER_1000);
        for n in 0..100_000 {
            assert_eq!(sieve.is_prime(n), UNDER_100000.contains(&n));
        }
    }
}
