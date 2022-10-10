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

fn build_range(input: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("1 3 7 12 14 21", build_range("1,3,7,2,4,1"));
        assert_eq!("1 2 3 11 12", build_range("1-3,1-2"));
        assert_eq!("1 3 5 10 11 12 13 14", build_range("1:5:2,10:4:1"));
        assert_eq!("104 105 106 107 108 109 110 111 112", build_range("104-2"));
    }
}
