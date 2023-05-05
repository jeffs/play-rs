use std::borrow::Cow;
pub struct FizzBuzz(i32);

impl Iterator for FizzBuzz {
    type Item = Cow<'static, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(if self.0 % 15 == 0 {
            Cow::from("FizzBuzz")
        } else if self.0 % 5 == 0 {
            Cow::from("Buzz")
        } else if self.0 % 3 == 0 {
            Cow::from("Fizz")
        } else {
            Cow::from(self.0.to_string())
        })
    }
}

pub fn fizz_buzz() -> FizzBuzz {
    FizzBuzz(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        for (got, want) in fizz_buzz().zip([
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz", "16",
        ]) {
            assert_eq!(got, want);
        }
    }
}
