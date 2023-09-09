use std::env;

use play_rs::primes::{self, primes, Cache as PrimeCache};

#[derive(Clone, Copy, Default)]
enum LogLevel {
    Some,
    #[default]
    None,
}

struct Args {
    log_level: LogLevel,
    use_cache: bool,
    targets: Vec<u32>,
}

fn parse_args() -> Args {
    let default_target = 2147483647;
    let mut log_level = LogLevel::default();
    let mut use_cache = false;
    let mut targets: Vec<u32> = Vec::new();
    for arg in env::args().skip(1) {
        if arg == "--log" {
            log_level = LogLevel::Some
        } else if arg == "--cache" {
            use_cache = true;
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
        use_cache,
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
        use_cache,
        targets,
    } = parse_args();
    for target in targets {
        let option = if use_cache {
            let cache = PrimeCache::new(&primes::UNDER_100000);
            search(cache.primes(), target, log_level)
        } else {
            search(primes(), target, log_level)
        };
        if let Some(index) = option {
            println!("{target} found at index {index}");
        } else {
            println!("{target} is not prime");
        }
    }
}
