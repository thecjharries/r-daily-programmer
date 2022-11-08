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

fn is_prime(number: u32) -> bool {
    if number < 2 {
        return false;
    } else if number == 2 {
        return true;
    } else if 0 == number % 2 {
        return false;
    }
    for divisor in (3..(number as f32).sqrt() as u32 + 1).step_by(2) {
        if 0 == number % divisor {
            return false;
        }
    }
    true
}

fn find_prime_range(number: u32) -> (u32, u32) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(false, is_prime(1));
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(false, is_prime(4));
        assert_eq!(true, is_prime(5));
        assert_eq!(false, is_prime(6));
        assert_eq!(true, is_prime(7));
        assert_eq!(false, is_prime(8));
        assert_eq!(false, is_prime(9));
        assert_eq!(false, is_prime(10));
    }
}
