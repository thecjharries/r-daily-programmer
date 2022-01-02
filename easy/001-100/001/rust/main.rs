use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let mut name = String::new();
    let mut age = String::new();
    let mut reddit_name = String::new();
    let std = io::stdin();
    let mut handle = std.lock();

    print!("What is your name? ");
    io::stdout().flush().unwrap();
    handle.read_line(&mut name)?;
    print!("What is your age? ");
    io::stdout().flush().unwrap();
    handle.read_line(&mut age)?;
    print!("What is your reddit username? ");
    io::stdout().flush().unwrap();
    handle.read_line(&mut reddit_name)?;

    println!("your name is {}, you are {} years old, and your username is {}", name.trim_end(), age.trim_end(), reddit_name.trim_end());
    Ok(())
}
