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

fn nth_fibonacci(order: usize, element: usize) -> usize {

    if element < order - 1 {
        return 0;
    } else if element == order - 1 {
        return 1;
    }
    let mut fibonacci = vec![0; order - 1];
    fibonacci.push(1);
    for _ in order..=element {
        fibonacci.push(fibonacci[fibonacci.len() - order..fibonacci.len()].iter().sum());
    }
    *fibonacci.last().unwrap()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_fibonacci() {
        assert_eq!(0, nth_fibonacci(3, 0));
        assert_eq!(0, nth_fibonacci(3, 1));
        assert_eq!(1, nth_fibonacci(3, 2));
        assert_eq!(1, nth_fibonacci(3, 3));
        assert_eq!(2, nth_fibonacci(3, 4));
        assert_eq!(4, nth_fibonacci(3, 5));
        assert_eq!(7, nth_fibonacci(3, 6));
        assert_eq!(13, nth_fibonacci(3, 7));
    }
}
