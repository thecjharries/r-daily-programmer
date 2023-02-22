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

use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn foil_binomial(input: &str) -> String {
    let binding = input.replace(" ", "");
    let mut terms: Vec<&str> = binding.split(")(").collect();
    terms[0] = terms[0].trim_start_matches('(');
    terms[1] = terms[1].trim_end_matches(')');
    let mut output = String::new();
    let mut coefficients: HashMap<u32, u32> = HashMap::new();
    for first in terms[0].split("+") {
        let first_binding = first.replace("^", "");
        let first_terms: Vec<&str> = first_binding.split("x").collect();
        let coefficient = first_terms[0].parse::<u32>().unwrap();
        let exponent = if first.contains("x") {
            if first_terms.len() > 1 {
                first_terms[1].parse::<u32>().unwrap_or(1)
            } else {
                1
            }
        } else {
            0
        };
        for second in terms[1].split("+") {
            let second_binding = second.replace("^", "");
            let second_terms: Vec<&str> = second_binding.split("x").collect();
            let second_coefficient = second_terms[0].parse::<u32>().unwrap();
            let second_exponent = if second.contains("x") {
                if second_terms.len() > 1 {
                    second_terms[1].parse::<u32>().unwrap_or(1)
                } else {
                    1
                }
            } else {
                0
            };
            let product = coefficient * second_coefficient;
            let sum = exponent + second_exponent;
            coefficients.insert(sum, coefficients.get(&sum).unwrap_or(&0) + product);
        }
    }
    let mut sorted_exponents: Vec<u32> = coefficients.keys().copied().collect();
    sorted_exponents.sort();
    sorted_exponents.reverse();
    for exponent in sorted_exponents {
        let mut coefficient = coefficients.get(&exponent).unwrap().to_string();
        if String::from("1") == coefficient {
            coefficient = String::from("");
        }
        let x_term = if 0 == exponent {
            String::from("")
        } else if 1 == exponent {
            String::from("x")
        } else {
            format!("x^{}", exponent)
        };
        output.push_str(&format!("{}{}", coefficient, x_term));
        if exponent > 0 {
            output.push_str(" + ");
        }
    }
    output.trim_end_matches(" + ").to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "14x^2 + 48x + 18".to_string(),
            foil_binomial("(2x + 6)(7x + 3)")
        );
        assert_eq!(
            "10x^4 + 33x^3 + 27x^2".to_string(),
            foil_binomial("(2x^2 + 3x)(5x^2 + 9x)")
        );
    }
}
