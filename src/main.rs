use std::process::exit;

use play_rs::primal::Number;

fn main() {
    let six = Number::from_json_file("data/primal_six.json").unwrap_or_else(|err| {
        eprintln!("error: {err}");
        exit(1);
    });

    dbg!(six);
}
