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

fn limited_demorgan_processor(input: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limited_demorgan_processor_handles_limited_examples() {
        assert_eq!("", limited_demorgan_processor(""));
        assert_eq!("NOT a", limited_demorgan_processor("a"));
        assert_eq!("NOT a", limited_demorgan_processor("NOT NOT a"));
        assert_eq!("a", limited_demorgan_processor("NOT a"));
        assert_eq!("NOT a OR NOT b", limited_demorgan_processor("a AND b"));
        assert_eq!("NOT a AND NOT b", limited_demorgan_processor("a OR b"));
        // assert_eq!("a AND b", limited_demorgan_processor("NOT (a OR b)"));
    }
}
