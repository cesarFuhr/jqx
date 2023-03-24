use is_terminal::IsTerminal;
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

    println!("{}", buf);

    Ok(())
}
