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

use std::collections::BTreeMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn simplify_units(input: &str) -> String {
    let exploded = input.split('*').collect::<Vec<&str>>();
    let mut denominator = BTreeMap::new();
    let mut numerator = BTreeMap::new();
    for unit in exploded {
        let exploded_unit = unit.split('/').collect::<Vec<&str>>();
        if 1 == exploded_unit.len() {
            for unit in exploded_unit[0].chars() {
                let count = numerator.entry(unit).or_insert(0);
                *count += 1;
            }
        } else {
            for unit in exploded_unit[0].chars() {
                let count = numerator.entry(unit).or_insert(0);
                *count += 1;
            }
            for unit in exploded_unit[1].chars() {
                let count = denominator.entry(unit).or_insert(0);
                *count += 1;
            }
        }
    }
    for (unit, count) in denominator {
        let numerator_count = numerator.entry(unit).or_insert(0);
        *numerator_count -= count;
    }
    let mut result = String::new();
    for (unit, count) in numerator {
        if 0 != count {
            result.push(unit);
            if 1 != count {
                result.push_str(&format!("^{}", count));
            }
            result.push(' ');
        }
    }
    result.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_units() {
        assert_eq!("c d m^2 s^-1", simplify_units("m/s*m*cd"));
    }
}
