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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_square(size: usize) -> String {
    let mut lines = Vec::new();
    for _ in 0..size {
        lines.push("■".repeat(size));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_square() {
        assert_eq!("", build_square(0));
        assert_eq!("■", build_square(1));
        assert_eq!("■■\n■■", build_square(2));
        assert_eq!("■■■\n■■■\n■■■", build_square(3));
        assert_eq!("■■■■\n■■■■\n■■■■\n■■■■", build_square(4));
        assert_eq!("■■■■■\n■■■■■\n■■■■■\n■■■■■\n■■■■■", build_square(5));
    }
}
