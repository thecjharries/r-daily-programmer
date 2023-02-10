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

use rgsl::polynomials::cubic_equations::poly_solve_cubic;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_tetrahedral_number_base(input: u64) -> u64 {
    if 0 == input {
        return 0;
    }
    if 1 == input {
        return 1;
    }
    let (_, x_0, _, _) = poly_solve_cubic(3.0, 2.0, -6.0 * (input as f64));
    if x_0.ceil() == x_0.floor() {
        return ((x_0 * (x_0 + 1.0) * (x_0 + 2.0) / 6.0) - ((x_0 - 1.0) * x_0 * (x_0 + 1.0) / 6.0))
            as u64;
    }
    ((x_0.ceil() * (x_0.ceil() + 1.0) * (x_0.ceil() + 2.0) / 6.0)
        - (x_0.floor() * (x_0.floor() + 1.0) * (x_0.floor() + 2.0) / 6.0)) as u64
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_tetrahedral_number_base() {
        assert_eq!(0, find_tetrahedral_number_base(0));
        assert_eq!(1, find_tetrahedral_number_base(1));
        assert_eq!(3, find_tetrahedral_number_base(2));
        assert_eq!(3, find_tetrahedral_number_base(3));
        assert_eq!(3, find_tetrahedral_number_base(4));
        assert_eq!(6, find_tetrahedral_number_base(5));
        assert_eq!(6, find_tetrahedral_number_base(6));
        assert_eq!(6, find_tetrahedral_number_base(7));
        assert_eq!(6, find_tetrahedral_number_base(8));
        assert_eq!(
            505013002528,
            find_tetrahedral_number_base(169179692512835000)
        );
    }
}
