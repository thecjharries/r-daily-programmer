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

struct Polynomial {
    coefficients: Vec<f32>,
}

impl Polynomial {
    fn new(coefficients: Vec<f32>) -> Polynomial {
        Polynomial { coefficients }
    }

    fn degree(&self) -> usize {
        for (index, coefficient) in self.coefficients.iter().enumerate().rev() {
            if 0.0 != *coefficient {
                return index;
            }
        }
    }

    fn divide(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        let mut quotient = Polynomial::new(vec![0.0; self.degree() - divisor.degree() + 1]);
        let mut remainder = self.clone();
        while remainder.degree() >= divisor.degree() {
            let degree_difference = remainder.degree() - divisor.degree();
            let coefficient =
                remainder.coefficients[remainder.degree()] / divisor.coefficients[divisor.degree()];
            quotient.coefficients[degree_difference] = coefficient;
            let mut new_remainder = Polynomial::new(vec![0.0; degree_difference]);
            for (index, coefficient) in divisor.coefficients.iter().enumerate() {
                new_remainder.coefficients[index] = coefficient * coefficient;
            }
            remainder = remainder - new_remainder;
        }
        (quotient, remainder)
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn divide_polynomials(numerator: Vec<f32>, denominator: Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(
            divide_polynomials(vec![3., -6., 2., 4.], vec![-3., 1.]),
            (vec![36., 14., 4.], vec![111.]),
        );
    }
}
