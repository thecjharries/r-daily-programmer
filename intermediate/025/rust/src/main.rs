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

fn decimal_to_binary(decimal: u32) -> String {
    if 0 == decimal {
        return "0".to_string();
    }
    let mut binary = String::new();
    let mut remainder = decimal;
    while 0 < remainder {
        binary = format!("{}{}", remainder % 2, binary);
        remainder /= 2;
    }
    binary
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_binary() {
        assert_eq!("0", decimal_to_binary(0));
        assert_eq!("1", decimal_to_binary(1));
        assert_eq!("10", decimal_to_binary(2));
        assert_eq!("11", decimal_to_binary(3));
        assert_eq!("100", decimal_to_binary(4));
        assert_eq!("101", decimal_to_binary(5));
        assert_eq!("110", decimal_to_binary(6));
        assert_eq!("111", decimal_to_binary(7));
        assert_eq!("1000", decimal_to_binary(8));
    }
}
