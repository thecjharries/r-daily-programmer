use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let std = io::stdin();
    let mut handle = std.lock();

    handle.read_line(&mut buffer)?;

    println!("{}", buffer);
    Ok(())
}
