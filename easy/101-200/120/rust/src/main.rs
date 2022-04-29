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

use std::io::{BufRead, Read};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    println!("rad");
}

fn count_lines_duration(input: &mut (impl Read + BufRead), seconds: u64) -> u64 {
    let (tx, rx) = channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(seconds));
        tx.send(true).unwrap();
    });
    let mut count = 0;
    for _ in input.lines() {
        if let Ok(true) = rx.try_recv() {
            break;
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            count_lines_duration(&mut "one\ntwo\nthree\n".as_bytes(), 1),
            3
        );
    }
}
