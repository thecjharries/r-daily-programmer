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

fn numcompare(first: &str, second: &str) -> bool {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(false, numcompare("I", "I"));
        assert_eq!(true, numcompare("I", "II"));
        assert_eq!(false, numcompare("II", "I"));
        assert_eq!(false, numcompare("V", "IIII"));
        assert_eq!(true, numcompare("MDCLXV", "MDCLXVI"));
        assert_eq!(false, numcompare("MM", "MDCCCCLXXXXVIIII"));
    }
}
