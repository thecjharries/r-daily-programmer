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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn create_cuboid(width: usize, height: usize, depth: usize) -> String {
    let mut output = String::new();
    for index in 0..depth {
        output.push_str(&format!(
            "{}{}/{}\n",
            " ".repeat(depth - index),
            ":".repeat(width - 1),
            "+".repeat(index),
        ));
    }
    for _ in 0..height - depth - 1 {
        output.push_str(&format!("{}{}\n", "#".repeat(width), "+".repeat(depth),));
    }
    for index in 0..=depth {
        output.push_str(&format!(
            "{}{}\n",
            "#".repeat(width),
            "+".repeat(depth - index),
        ));
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cuboid() {
        let output = "   :::::::::::::::::::/
  :::::::::::::::::::/+
 :::::::::::::::::::/++
####################+++
####################+++
####################+++
####################+++
####################+++
####################+++
####################+++
####################++
####################+
####################
"
        .to_string();
        assert_eq!(output, create_cuboid(20, 10, 3));
    }
}
