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

#[derive(PartialEq, Debug)]
struct LoanConsiderations {
    interest_rate: f64,
    annual_loan_amount: f64,
    start_age: f64,
    clawback_balance_trigger: f64,
    royalty_rate_under_65: f64,
    royalty_rate_over_65: f64,
    income_stream_thousands: Vec<f64>,
}

#[derive(PartialEq, Debug)]
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
        let mut current_royalty_rate = input.royalty_rate_under_65;
        for (age_index, income) in input.income_stream_thousands.iter().enumerate() {
            if 65.0 <= input.start_age + age_index as f64 {
                current_royalty_rate = input.royalty_rate_over_65;
            }
            results.overall_loans_taken += input.annual_loan_amount;
            results.ending_balance_with_interest = results.ending_balance_with_interest
                * (1.0 + input.interest_rate / 100.0)
                + input.annual_loan_amount;
            let current_annual_income = income * 1000.0;
            let current_royalty = current_annual_income * current_royalty_rate / 100.0;
            let mut current_clawback = 0.0;
            if input.clawback_balance_trigger <= results.ending_balance_with_interest {
                current_clawback = current_royalty_rate * input.annual_loan_amount / 100.0;
            }
            results.repayments_from_income += current_royalty;
            results.repayments_from_benefits_clawbacks += current_clawback;
            results.ending_balance_with_interest -= current_royalty + current_clawback;
        }
        results.overall_loans_taken = (results.overall_loans_taken / 1000.0).round();
        results.repayments_from_income = (results.repayments_from_income / 1000.0).round();
        results.repayments_from_benefits_clawbacks =
            (results.repayments_from_benefits_clawbacks / 1000.0).round();
        results.ending_balance_with_interest =
            (results.ending_balance_with_interest / 1000.0).round();
        results
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
    fn test_loanresults_compute() {
        let first_input = LoanConsiderations {
            interest_rate: 2.0,
            annual_loan_amount: 15000.0,
            start_age: 18.0,
            clawback_balance_trigger: 100000.0,
            royalty_rate_under_65: 20.0,
            royalty_rate_over_65: 40.0,
            income_stream_thousands: vec![
                0.0, 0.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 30.0, 30.0,
                30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 40.0, 40.0, 40.0, 40.0, 40.0, 40.0,
                40.0, 40.0, 40.0, 40.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        };
        let first_expected = LoanResults {
            overall_loans_taken: 1080.0,
            repayments_from_income: 280.0,
            repayments_from_benefits_clawbacks: 270.0,
            ending_balance_with_interest: 1169.0,
        };
        assert_eq!(first_expected, LoanResults::compute(first_input));
        let second_input = LoanConsiderations {
            interest_rate: 2.0,
            annual_loan_amount: 15000.0,
            start_age: 18.0,
            clawback_balance_trigger: 100000.0,
            royalty_rate_under_65: 20.0,
            royalty_rate_over_65: 40.0,
            income_stream_thousands: vec![
                0.0, 0.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 40.0, 40.0,
                40.0, 40.0, 40.0, 40.0, 40.0, 40.0, 40.0, 40.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0,
                50.0, 50.0, 50.0, 50.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0,
                100.0, 120.0, 140.0, 160.0, 200.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,
                10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,
            ],
        };
        let second_expected = LoanResults {
            overall_loans_taken: 1005.0,
            repayments_from_income: 584.0,
            repayments_from_benefits_clawbacks: 237.0,
            ending_balance_with_interest: 509.0,
        };
        assert_eq!(second_expected, LoanResults::compute(second_input));
    }
}
