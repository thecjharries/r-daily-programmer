// Copyright 2022 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
