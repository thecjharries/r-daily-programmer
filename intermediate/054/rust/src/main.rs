// Copyright 2023 CJ Harries
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

use std::io::{BufRead, BufReader};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn handle_client(stream: impl Read + Write, output: &mut impl Write) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        let line = line.unwrap();
        output.write_all(line.as_bytes()).unwrap();
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use mockstream::MockStream;

    #[test]
    fn test_handle_client() {
        let mut stream = MockStream::new();
        stream.push_bytes_to_read(b"hello");
        let mut output = Vec::new();
        handle_client(stream, &mut output);
        assert_eq!(b"hello", output.as_slice());
    }
}
