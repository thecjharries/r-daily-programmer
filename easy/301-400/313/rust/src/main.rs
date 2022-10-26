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

use itertools::Itertools;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn has_zero_combination(input: Vec<i32>) -> bool {
    if input.contains(&0) {
        return true;
    }
    for length in 2..=input.len() {
        for combination in input.iter().combinations(length).unique() {
            if combination.into_iter().sum::<i32>() == 0 {
                return true;
            }
        }
    }
    false
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_zero_combination() {
        assert_eq!(true, has_zero_combination(vec![0]));
        assert_eq!(true, has_zero_combination(vec![-3, 1, 2]));
        assert_eq!(
            true,
            has_zero_combination(vec![
                -98634, -86888, -48841, -40483, 2612, 9225, 17848, 71967, 84319, 88875
            ])
        );
        assert_eq!(
            false,
            has_zero_combination(vec![
                -83314, -82838, -80120, -63468, -62478, -59378, -56958, -50061, -34791, -32264,
                -21928, -14988, 23767, 24417, 26403, 26511, 36399, 78055
            ])
        );
        assert_eq!(
            false,
            has_zero_combination(vec![
                -92953, -91613, -89733, -50673, -16067, -9172, 8852, 30883, 46690, 46968, 56772,
                58703, 59150, 78476, 84413, 90106, 94777, 95148
            ])
        );
        assert_eq!(
            false,
            has_zero_combination(vec![
                -94624, -86776, -85833, -80822, -71902, -54562, -38638, -26483, -20207, -1290,
                12414, 12627, 19509, 30894, 32505, 46825, 50321, 69294
            ])
        );
        assert_eq!(
            false,
            has_zero_combination(vec![
                -83964, -81834, -78386, -70497, -69357, -61867, -49127, -47916, -38361, -35772,
                -29803, -15343, 6918, 19662, 44614, 66049, 93789, 95405
            ])
        );
        assert_eq!(
            false,
            has_zero_combination(vec![
                -68808, -58968, -45958, -36013, -32810, -28726, -13488, 3986, 26342, 29245, 30686,
                47966, 58352, 68610, 74533, 77939, 80520, 87195
            ])
        );
    }
}
