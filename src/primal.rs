//! Module for working with numbers represented as products of powers of primes.
//! I'm still playing with the notation, and figuring out how to implement
//! traditional arithmetic operations on it.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    primes,
    simple_error::{BoxedError, StringError},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Number {
    Zero,
    List(Vec<Number>),
}

impl Number {
    pub fn from_json(value: Value) -> Result<Number, StringError> {
        // TODO: As a convenience, parse base 10 into primal numbers.  For
        // example, turn 37 into [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, []].
        let number = match value {
            Value::Number(n) if n.as_u64() == Some(0) => Number::Zero,
            Value::Array(ns) => {
                let components: Result<Vec<Number>, StringError> =
                    ns.into_iter().map(Number::from_json).collect();
                Number::List(components?)
            }
            _ => return Err(StringError("bad number".to_string())),
        };
        Ok(number)
    }

    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Number, BoxedError> {
        let text = fs::read_to_string(path).expect("input file should be readable");
        Ok(Number::from_json(serde_json::from_str(&text)?)?)
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            // TODO: Generate primes as needed.
            Number::Zero => 0,
            Number::List(powers) => primes::UNDER_1000
                .into_iter()
                .zip(powers)
                .map(|(prime, power)| prime.pow(power.to_u32()))
                .product(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn to_u32(value: Value) -> u32 {
        Number::from_json(value)
            .expect("test JSON must be primal")
            .to_u32()
    }

    fn file_to_u32(path: &str) -> u32 {
        Number::from_json_file(path)
            .expect("test file should be readable")
            .to_u32()
    }

    #[test]
    fn test_one() {
        assert_eq!(to_u32(json!([])), 1);
    }

    #[test]
    fn test_two() {
        assert_eq!(to_u32(json!([[]])), 2);
    }

    #[test]
    fn test_six() {
        assert_eq!(file_to_u32("data/primal_six.json"), 6);
    }

    #[test]
    fn test_ten() {
        assert_eq!(file_to_u32("data/primal_ten.json"), 10);
    }

    #[test]
    fn test_72() {
        let value = json!([
            [0, []], // 2 pow 3
            [[]],    // 3 pow 2
        ]);
        assert_eq!(to_u32(value), 72);
    }

    #[test]
    fn test_100() {
        let value = json!([
            [[]], // 2 pow 2
            0,    // 3 pow 0
            [[]], // 5 pow 2
        ]);
        assert_eq!(to_u32(value), 100);
    }
}
