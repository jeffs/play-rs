use std::env;

use play_rs::primes::{self, primes, Cache as PrimeCache, Sieve};

#[derive(Clone, Copy, Default)]
enum LogLevel {
    Some,
    #[default]
    None,
}

#[derive(Default)]
enum Algorithm {
    Stateless,
    Cache,
    #[default]
    Sieve,
}

struct Args {
    log_level: LogLevel,
    algorithm: Algorithm,
    targets: Vec<u32>,
}

fn parse_args() -> Args {
    let default_target = 2147483647;
    let mut log_level = LogLevel::default();
    let mut algorithm = Algorithm::default();
    let mut targets: Vec<u32> = Vec::new();
    for arg in env::args().skip(1) {
        if arg == "--log" {
            log_level = LogLevel::Some
        } else if arg == "--stateless" {
            algorithm = Algorithm::Stateless;
        } else if arg == "--cache" {
            algorithm = Algorithm::Cache;
        } else if arg == "--sieve" {
            algorithm = Algorithm::Sieve;
        } else {
            let message = format!("{arg}: expected natural number");
            targets.push(arg.parse().expect(&message));
        }
    }
    if targets.is_empty() {
        targets.push(default_target);
    }
    Args {
        log_level,
        algorithm,
        targets,
    }
}

fn search(primes: impl Iterator<Item = u32>, target: u32, log_level: LogLevel) -> Option<usize> {
    primes
        .take_while(|&n| n <= target)
        .enumerate()
        .inspect(|(i, n)| {
            if let LogLevel::Some = log_level {
                if i % 1000 == 0 {
                    eprintln!("{i:8} {n}");
                }
            }
        })
        .last()
        .filter(|&(_, value)| value == target)
        .map(|(index, _)| index)
}

fn main() {
    let Args {
        log_level,
        algorithm,
        targets,
    } = parse_args();
    for target in targets {
        let found = match algorithm {
            Algorithm::Stateless => search(primes(), target, log_level),
            Algorithm::Cache => {
                let cache = PrimeCache::new(&primes::UNDER_100000);
                search(cache.primes(), target, log_level)
            }
            Algorithm::Sieve => search(Sieve::default().into_primes(), target, log_level),
        };
        if let Some(index) = found {
            println!("{target} found at index {index}");
        } else {
            println!("{target} is not prime");
        }
    }
}
