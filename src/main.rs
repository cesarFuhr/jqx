use is_terminal::IsTerminal;
use serde_json::Value;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    if std::io::stdin().is_terminal() {
        println!("Stdin is a terminal");
    } else {
        println!("Stdin is not a terminal");
    }

    let mut stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf)?;

    let v: Value = serde_json::from_str(&buf).unwrap();

    println!("{}", serde_json::to_string_pretty(&v).unwrap());

    Ok(())
}
