use std::{mem, ops::AddAssign};

#[derive(Clone)]
struct Fibs<T>(T, T);

impl<T: Copy> Copy for Fibs<T> {}

impl<T: AddAssign + Clone> Iterator for Fibs<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let old = self.clone();
        mem::swap(&mut self.0, &mut self.1);
        self.1 += old.1;
        Some(old.0)
    }
}

fn fibs() -> impl Iterator<Item = u32> {
    Fibs(1, 1)
}

fn main() {
    for (got, want) in fibs().take(5).zip([1, 1, 2, 3, 5]) {
        assert_eq!(got, want);
    }
}
