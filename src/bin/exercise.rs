use core::fmt;
use std::{
    env,
    error::Error,
    io,
    num::{ParseFloatError, ParseIntError},
    process::exit,
    str::FromStr,
};

#[derive(Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}

impl From<ParseFloatError> for ParseError {
    fn from(cause: ParseFloatError) -> Self {
        ParseError(cause.to_string())
    }
}
impl From<ParseIntError> for ParseError {
    fn from(cause: ParseIntError) -> Self {
        ParseError(cause.to_string())
    }
}

struct Exercise {
    sets: u16,
    reps: u16, // per set
    lbs: f32,
}

impl Exercise {
    fn volume(&self) -> f32 {
        self.sets as f32 * self.reps as f32 * self.lbs
    }
}

impl FromStr for Exercise {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((sets, s)) = s.split_once('x') else {
            return Err(ParseError("expected x".into()));
        };
        Ok(match s.split_once('@') {
            Some((reps, lbs)) => Exercise {
                sets: sets.parse()?,
                reps: reps.parse()?,
                lbs: lbs.parse()?,
            },
            None => Exercise {
                sets: sets.parse()?,
                reps: s.parse()?,
                lbs: 1.0,
            },
        })
    }
}

fn show_exercise(word: &str) -> Result<String, ParseError> {
    let exercise: Exercise = word.parse()?;
    let volume = exercise.volume();
    Ok(format!("{word}={volume}"))
}

fn print_exercise(word: impl AsRef<str>) -> Result<(), ParseError> {
    println!("{}", show_exercise(word.as_ref())?);
    Ok(())
}

fn main_imp() -> Result<(), Box<dyn Error>> {
    for word in env::args().skip(1) {
        print_exercise(word)?;
    }
    for line in io::stdin().lines() {
        for word in line?.split_ascii_whitespace() {
            print_exercise(word)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        eprintln!("error: {err}");
        exit(1);
    }
}
