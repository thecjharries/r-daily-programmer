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

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn main() {
    println!("rad");
}

fn count_lines_in_file(file_path: &Path) -> i32 {
    let file = BufReader::new(File::open(file_path).expect("Unable to open file"));
    let mut count = 0;
    for _ in file.lines() {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_in_file() {
        let file_path = Path::new("Cargo.toml");
        assert_eq!(11, count_lines_in_file(file_path));
    }
}
