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

use std::io::Write;
use std::os::unix::net::UnixStream;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn handle_client(mut stream: UnixStream, output: &mut impl Write) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_client() {
        let mut stream = UnixStream::connect("/tmp/054.sock").unwrap();
        stream.write_all(b"hello").unwrap();
        let mut output = Vec::new();
        handle_client(stream, &mut output);
        assert_eq!(b"olleh", output.as_slice());
    }
}
