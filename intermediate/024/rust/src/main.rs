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

fn continued_fraction(n: u32, m: u32) -> (u32, u32) {
    if n < m {
        let (denominator, b) = continued_fraction(n + 1, m);
        let numerator = (2 * n - 1) * denominator + n.pow(2) * b;
        return (numerator, denominator);
    }
    (1, 1)
}

fn pi(precision: u32) -> (u32, u32) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continued_fraction() {
        assert_eq!((1, 1), continued_fraction(1, 1));
        assert_eq!((1, 1), continued_fraction(10, 1));
        assert_eq!((2, 1), continued_fraction(1, 2));
    }

    #[test]
    fn test_stub() {
        assert_eq!((4, 1), pi(1));
        assert_eq!((4, 2), pi(2));
        assert_eq!((28, 8), pi(3));
        assert_eq!((184, 60), pi(4));
    }
}
