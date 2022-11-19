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

#[derive(Debug, PartialEq, Clone)]
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
        0
    }

    fn divide(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        let mut remainder = self.clone();
        if divisor.degree() > self.degree() {
            return (Polynomial::new(vec![0.0]), remainder);
        }
        let mut quotient = Polynomial::new(vec![0.0; self.degree() - divisor.degree() + 1]);
        while remainder.degree() >= divisor.degree() {
            let mut denominator = Polynomial::new(vec![0.0; remainder.degree() + 1]);
            for (index, coefficient) in divisor.coefficients
                [remainder.degree() - divisor.degree()..]
                .iter()
                .enumerate()
            {
                denominator.coefficients[index] = *coefficient;
            }
            quotient.coefficients[remainder.degree() - divisor.degree()] =
                remainder.coefficients[remainder.degree()] / denominator.coefficients[0];
            for index in 0..denominator.degree() {
                denominator.coefficients[index] *=
                    quotient.coefficients[remainder.degree() - divisor.degree()];
                remainder.coefficients[index] -= denominator.coefficients[index];
            }
        }
        (quotient, remainder.clone())
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn divide_polynomials(numerator: Vec<f32>, denominator: Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    let numerator_polynomial = Polynomial::new(numerator);
    let denominator_polynomial = Polynomial::new(denominator);
    let (quotient, remainder) = numerator_polynomial.divide(&denominator_polynomial);
    (quotient.coefficients, remainder.coefficients)
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
