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

struct LoanConsiderations {
    interest_rate: f64,
    annual_loan_amount: f64,
    start_age: f64,
    clawback_balance_trigger: f64,
    royalty_rate_under_65: f64,
    royalty_rate_over_65: f64,
    income_stream_thousands: Vec<f64>,
}

struct LoanResults {
    overall_loans_taken: f64,
    repayments_from_income: f64,
    repayments_from_benefits_clawbacks: f64,
    ending_balance_with_interest: f64,
}

impl LoanResults {
    fn compute(input: LoanConsiderations) -> Self {
        let mut results = LoanResults {
            overall_loans_taken: 0.0,
            repayments_from_income: 0.0,
            repayments_from_benefits_clawbacks: 0.0,
            ending_balance_with_interest: 0.0,
        };
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
