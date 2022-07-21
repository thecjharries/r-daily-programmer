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

fn construct_rhyme(name: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "Lincoln, Lincoln bo Bincoln,
Bonana fanna fo Fincoln,
Fee fy mo Mincoln,
Lincoln!",
            construct_rhyme("Lincoln")
        );
        assert_eq!(
            "Nick, Nick bo Bick,
Bonana fanna fo Fick,
Fee fy mo Mick,
Nick!",
            construct_rhyme("Nick")
        );
    }
}
