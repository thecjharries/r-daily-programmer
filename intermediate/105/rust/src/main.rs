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

fn basic_bool_eval(input: &str) -> u8 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_bool_eval() {
        assert_eq!(1, basic_bool_eval("1"));
        assert_eq!(0, basic_bool_eval("0"));
        assert_eq!(1, basic_bool_eval("1 * 1"));
        assert_eq!(0, basic_bool_eval("1 * 0"));
        assert_eq!(0, basic_bool_eval("0 * 1"));
        assert_eq!(0, basic_bool_eval("0 * 0"));
        assert_eq!(1, basic_bool_eval("1 | 1"));
        assert_eq!(1, basic_bool_eval("1 | 0"));
        assert_eq!(1, basic_bool_eval("0 | 1"));
        assert_eq!(0, basic_bool_eval("0 | 0"));
        assert_eq!(1, basic_bool_eval("1 ^ 0"));
        assert_eq!(1, basic_bool_eval("0 ^ 1"));
        assert_eq!(0, basic_bool_eval("1 ^ 1"));
        assert_eq!(0, basic_bool_eval("0 ^ 0"));
        assert_eq!(1, basic_bool_eval("!0"));
        assert_eq!(0, basic_bool_eval("!1"));
    }
}
