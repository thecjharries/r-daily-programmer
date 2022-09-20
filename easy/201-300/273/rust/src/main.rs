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

fn convert(input: &str) -> String {
    let number = input
        .replace("r", "")
        .replace("d", "")
        .parse::<f64>()
        .unwrap();
    if input.ends_with("d") {
        format!("{}d", number * 180.0 / std::f64::consts::PI)
    } else {
        format!("{}r", number * std::f64::consts::PI / 180.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!("180.0004209182994d", convert("3.1416rd"));
        assert_eq!("1.5707963267948966r", convert("90dr"));
    }
}
