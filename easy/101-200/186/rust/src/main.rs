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

use std::process::Command;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn get_running_processes() -> Vec<String> {
    let output = Command::new("ps")
        .arg("aux")
        .output()
        .expect("failed to execute process");
    let mut processes = Vec::new();
    for line in &String::from_utf8_lossy(&output.stdout)
        .lines()
        .collect::<Vec<_>>()[1..]
    {
        processes.push(line.to_string());
    }
    processes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_running_processes() {
        assert!(0 < get_running_processes().len());
    }
}
