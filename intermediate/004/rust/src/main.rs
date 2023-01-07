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

use eval::{eval, to_value};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn calculate(equation: &str) -> f64 {
    eval(equation).unwrap().as_f64().unwrap()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(to_value(6.0), calculate("1 + 2 + 3"));
        assert_eq!(to_value(7.0), calculate("2 * 2 + 3"));
        assert_eq!(to_value(4.0), calculate("2 / 2 + 3"));
        assert_eq!(to_value(2.0), calculate("2 / 2 + 3 / 3"));
    }
}
