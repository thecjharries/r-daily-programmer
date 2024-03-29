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

fn main() {
    println!("rad");
}

fn build_op_table(op: &str, max: i32) -> String {
    let mut table = String::new();
    table.push_str(format!("{} |", op).as_str());
    for index in 0..max + 1 {
        table.push_str(format!(" {}", index).as_str());
    }
    table.push_str(format!("\n{}\n", "-".repeat((3 + 2 * (max + 1)) as usize)).as_str());
    for index in 0..max + 1 {
        table.push_str(format!("{} |", index).as_str());
        for second in 0..max + 1 {
            match op {
                "+" => table.push_str(format!(" {}", index + second).as_str()),
                "-" => table.push_str(format!(" {}", index - second).as_str()),
                "*" => table.push_str(format!(" {}", index * second).as_str()),
                "/" => table.push_str(format!(" {}", index / second).as_str()),
                _ => { /* do nothing */ }
            }
        }
        table.push_str("\n");
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_op_table() {
        assert_eq!(
            build_op_table("+", 2),
            "+ | 0 1 2\n---------\n0 | 0 1 2\n1 | 1 2 3\n2 | 2 3 4\n".to_string()
        );
        assert_eq!(
            build_op_table("*", 2),
            "* | 0 1 2\n---------\n0 | 0 0 0\n1 | 0 1 2\n2 | 0 2 4\n".to_string()
        );
    }
}
