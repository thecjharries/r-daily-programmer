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

fn convert_to_scientific(number: f64) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_scientific() {
        assert_eq!(
            convert_to_scientific(239487.0),
            "2.39487 x 10^5".to_string()
        );
        assert_eq!(convert_to_scientific(0.654), "6.54 x 10^-1".to_string());
    }
}
