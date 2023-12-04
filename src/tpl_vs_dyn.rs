
trait Thing {}

impl<T> Thing for T { }
    
pub fn static_identity<T>(x: &T) -> &T {
    x
}

#[cfg(test)]
mod  test {

}
