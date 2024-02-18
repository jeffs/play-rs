use std::any::TypeId;

trait Big {}
trait Red {}

impl Big for () {}
impl Red for () {}
fn clifford() -> impl Big + Red {}

#[allow(dead_code)] // We use name and type only for debug.
#[derive(Debug)]
struct Field {
    name: &'static str,
    typ: TypeId,
}

trait DataclassInstance {
    fn fields(&self) -> Vec<Field>;
}

struct Yes;

impl DataclassInstance for Yes {
    fn fields(&self) -> Vec<Field> {
        vec![Field {
            name: "name",
            typ: TypeId::of::<String>(),
        }]
    }
}

// struct No;

fn main() {
    println!("{:?}", Yes.fields());
    println!("{:p}", &clifford());
}
