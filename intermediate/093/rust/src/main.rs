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

fn coordinate_to_z_order(x: usize, y: usize, length: usize) -> usize {
    let mut x_binary = format!("{:b}", x);
    if x_binary.len() < length {
        x_binary = format!("{}{}", "0".repeat(length - x_binary.len()), x_binary);
    }
    let mut y_binary = format!("{:b}", y);
    if y_binary.len() < length {
        y_binary = format!("{}{}", "0".repeat(length - y_binary.len()), y_binary);
    }
    let mut z_order = String::new();
    for index in 0..length {
        z_order.push(y_binary.chars().nth(index).unwrap());
        z_order.push(x_binary.chars().nth(index).unwrap());
    }
    usize::from_str_radix(&z_order, 2).unwrap()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_to_z_order() {
        assert_eq!(2479, coordinate_to_z_order(19, 47, 6));
        assert_eq!(0, coordinate_to_z_order(0, 0, 10));
        assert_eq!(1, coordinate_to_z_order(1, 0, 10));
        assert_eq!(2, coordinate_to_z_order(0, 1, 10));
        assert_eq!(3, coordinate_to_z_order(1, 1, 10));
        assert_eq!(4, coordinate_to_z_order(2, 0, 10));
    }
}
