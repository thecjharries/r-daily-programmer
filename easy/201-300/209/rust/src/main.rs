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

use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_flairs(user_time_map: HashMap<String, f32>) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let input = HashMap::from_iter([
            ("bholzer".to_string(), 101.09),
            ("Cosmologicon".to_string(), 27.45),
            ("nint22".to_string(), 13.76),
            ("nooodl".to_string(), 7.29),
            ("nottoobadguy".to_string(), 74.56),
            ("oskar_s".to_string(), 39.90),
            ("Steve132".to_string(), 61.82),
        ]);
        assert_eq!(
            "nooodl: 52\nnint22: 53\nCosmologicon: 46\noskar_s: 47\nSteve132: 38\nnottoobadguy: 47\nbholzer: 33\n",
            build_flairs(input)
        );
    }
}
