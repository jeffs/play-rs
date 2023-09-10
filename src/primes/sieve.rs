type Word = u64;

const WORD_BITS: usize = Word::BITS as usize;

// The little-endian index of each 1 bit, times two and plus one, is prime.  For
// example, the first ten bits are:
//
//  bits:       1101101110
//  indexes:    9876543210
//
// Meaning:
//
//  index             number    prime?
//  -----             ------    ------
//    0     * 2 + 1 =    1        0
//    1     * 2 + 1 =    3        1
//    2     * 2 + 1 =    5        1
//    3     * 2 + 1 =    7        1
//    4     * 2 + 1 =    9        0
//    5     * 2 + 1 =   11        1
//    6     * 2 + 1 =   13        1
//    7     * 2 + 1 =   15        0
//    8     * 2 + 1 =   17        1
//    9     * 2 + 1 =   19        1
const FIRST_WORD: Word = 0x816d129a64b4cb6e;

pub struct SievePrimes {
    sieve: Sieve,
    known: u32,
}

impl Iterator for SievePrimes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.known = match self.known {
            0 => 2,
            2 => 3,
            _ => ((self.known + 2)..)
                .step_by(2)
                .find(|&n| self.sieve.is_prime(n))
                .expect("another prime"),
        };
        Some(self.known)
    }
}

#[derive(Default)]
pub struct Sieve {
    words: Vec<Word>,
}

// TODO: Store only odd bits.
impl Sieve {
    fn mark_nonprime_value(&mut self, value: u32) {
        if value % 2 == 0 {
            return; // We don't store bits for even numbers, anyway.
        }
        let index = (value / 2) as usize;
        self.words[index / WORD_BITS] &= !(1 << (index % WORD_BITS));
    }

    fn check_is_prime_value(&self, value: u32) -> bool {
        // We get twice as many bits per word by skipping even-indexed bits,
        // since no even numbers past 2 are prime.  We special-case 2.
        if value % 2 == 0 {
            return value == 2;
        }
        let index = (value / 2) as usize;
        self.words[index / WORD_BITS] & (1 << (index % WORD_BITS)) != 0
    }

    fn num_values(&self) -> u32 {
        (self.words.len() * WORD_BITS * 2) as u32
    }

    // fn set_bit(&mut self, index: usize) {
    //     debug_assert!(index % 2 != 0);
    //     let index = index / 2;
    //     self.words[index / WORD_BITS] |= 1 << (index % WORD_BITS);
    // }

    pub fn double(&mut self) {
        if self.words.is_empty() {
            self.words.push(FIRST_WORD);
            return;
        }
        let num_old_values = self.num_values() as u32;
        self.words.resize(self.words.len() * 2, !0); // Append a bunch of 1s.
        let num_new_values = self.num_values() as u32;
        for value in (3..num_old_values).step_by(2) {
            if !self.check_is_prime_value(value) {
                continue; // Skip non-prime.
            }
            let offset = value - num_old_values % value;
            for new_index in ((num_old_values + offset)..num_new_values).step_by(value as usize) {
                self.mark_nonprime_value(new_index); // New index is divisible by value.
            }
        }
    }

    pub fn is_prime(&mut self, value: u32) -> bool {
        while self.num_values() <= value {
            self.double();
        }
        self.check_is_prime_value(value)
    }

    pub fn into_primes(self) -> SievePrimes {
        SievePrimes {
            sieve: self,
            known: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::primes::{UNDER_1000, UNDER_100000};

    use super::*;

    #[test]
    fn test_sieve_is_prime() {
        let mut sieve = Sieve::default();
        let known: HashSet<_> = UNDER_100000.into_iter().collect();
        for n in 0..100_000 {
            assert_eq!(sieve.is_prime(n), known.contains(&n));
        }
    }

    #[test]
    fn test_sieve_primes() {
        let want = UNDER_1000;
        let sieve = Sieve::default();
        let got: Vec<_> = sieve.into_primes().take(want.len()).collect();
        assert_eq!(got, want);
    }
}
