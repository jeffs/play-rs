/// Wraps an iterator of type I by creating it on the first call to .next(). Creates the wrapped
/// iterator by calling F.
enum Delay<I: Iterator, F: FnOnce() -> I> {
    // New wraps an Option only so that we can take F out of it during the transition from New to
    // Old.  It feels like this shouldn't be necessary, but we're not allowed to move out of self in
    // the body of Iterator::next, which (like any FnMut) merely borrows (rather than owning) self.
    // See also StackOverflow:
    // <https://stackoverflow.com/questions/36557412/how-can-i-change-enum-variant-while-moving-the-field-to-the-new-variant>
    New(Option<F>),
    Old(I),
}

impl<I: Iterator, F: FnOnce() -> I> Iterator for Delay<I, F> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Delay::New(new) => {
                // Every Delay begins life as New(Some(_)).  On the first call to .next(), it is
                // replaced by Old(_).  The only region in which a Delay is New(None) is the
                // following few lines, after we take the function out of the option.
                let mut old = new.take().unwrap()();
                let next = old.next();
                *self = Delay::Old(old);
                next
            }
            Delay::Old(old) => old.next(),
        }
    }
}

/// Wraps an iterator created by `new` on the first  call to [.next()][Iterator::next()].
fn delay<I: Iterator, F: FnOnce() -> I>(new: F) -> Delay<I, F> {
    Delay::New(Some(new))
}

/// Returns an inefficient iterator over the Fibonacci numbers.
///
/// This is a Rust implementation of what the Haskell docs call the "canonical zipWith
/// implementation:"
///
/// <https://wiki.haskell.org/The_Fibonacci_sequence#Canonical_zipWith_implementation>
/// ```hs
/// fibs = 0 : 1 : zipWith (+) fibs (tail fibs)
/// ```
fn fibs() -> Box<dyn Iterator<Item = u32>> {
    Box::new(
        [0, 1]
            .into_iter()
            .chain(delay(fibs).zip(delay(fibs).skip(1)).map(|(x, y)| x + y)),
    )
}

fn main() {
    for (got, want) in fibs().zip([0, 1, 1, 2, 3, 5, 8, 13]) {
        assert_eq!(got, want);
    }
}
