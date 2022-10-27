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

fn build_min_max(input: Vec<u32>) -> (String, String) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            ("3469798283".to_string(), "8382796934".to_string()),
            build_min_max(vec![79, 82, 34, 83, 69])
        );
        assert_eq!(
            ("193413442071".to_string(), "714203434119".to_string()),
            build_min_max(vec![420, 34, 19, 71, 341])
        );
        assert_eq!(
            ("173246791".to_string(), "917463217".to_string()),
            build_min_max(vec![17, 32, 91, 7, 46])
        );
    }
}
