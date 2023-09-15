trait Named {
    fn name(&self) -> &str;
}

fn greet(named: &impl Named) -> String {
    format!("Hello, {}.", named.name())
}

struct Dog {
    name: String,
}

impl Dog {
    fn named(name: impl Into<String>) -> Dog {
        Dog { name: name.into() }
    }
}

impl Named for Dog {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let fido = Dog::named("Fido");

    println!("{}", greet(&fido));
}
