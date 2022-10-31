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

fn countdown(numbers: Vec<u32>, result: u32) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "25 - 9 * 7 * 7 + 100 - 3 = 881",
            countdown(vec![25, 100, 9, 7, 3, 7], 881)
        );
        assert_eq!(
            "3 + 3 * 7 + 1 * 6 - 8 = 250",
            countdown(vec![1, 3, 7, 6, 8, 3], 250)
        );
    }
}
