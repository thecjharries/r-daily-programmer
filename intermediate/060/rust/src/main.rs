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

fn build_smiley(columns: usize) -> String {
    let columns = match columns {
        usize::MIN..=15 => 16,
        1001..=usize::MAX => 1000,
        _ => columns,
    };
    let mut output = String::new();
    for _ in 0..columns / 8 {
        output.push('\n');
    }
    for _ in 0..columns / 8 {
        output.push_str(
            format!(
                "{}{}{}{}{}",
                " ".repeat(columns / 10),
                "#".repeat(columns / 10),
                " ".repeat(6 * columns / 10),
                "#".repeat(columns / 10),
                "\n"
            )
            .as_str(),
        );
    }
    for _ in 0..columns / 4 {
        output.push('\n');
    }
    for _ in 0..columns / 8 {
        output.push_str(
            format!(
                "{}{}\n",
                " ".repeat(columns / 10),
                "#".repeat(8 * columns / 10),
            )
            .as_str(),
        )
    }
    for _ in 0..columns / 8 {
        output.push('\n');
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_smiley() {
        assert_eq!(
            "\n\n #         #\n #         #\n\n\n\n\n ############\n ############\n\n\n",
            build_smiley(16)
        );
    }
}
