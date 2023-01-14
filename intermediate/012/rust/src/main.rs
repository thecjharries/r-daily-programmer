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

fn factor(number: u32) -> Vec<u32> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(vec![1], factor(1));
        assert_eq!(vec![1, 2], factor(2));
        assert_eq!(vec![1, 3], factor(3));
        assert_eq!(vec![1, 2, 2], factor(4));
        assert_eq!(vec![1, 5], factor(5));
        assert_eq!(vec![1, 2, 3], factor(6));
        assert_eq!(vec![1, 2, 7], factor(14));
    }
}
