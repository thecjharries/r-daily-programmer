// Copyright 2021 CJ Harries
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

package main

import (
	"fmt"
	"math"
)

type LoanConsiderations struct {
	InterestRate           float64
	AnnualLoanAmount       float64
	StartAge               float64
	ClawbackBalanceTrigger float64
	RoyaltyRateUnder65     float64
	RoyaltyRateOver65      float64
	IncomeStreamThousands  []float64
}

type LoanResults struct {
	OverallLoansTaken               float64
	RepaymentsFromIncome            float64
	RepaymentsFromBenefitsClawbacks float64
	EndingBalanceWithInterest       float64
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func calculateLoanResults(input LoanConsiderations) (output LoanResults) {
	currentRoyaltyRate := input.RoyaltyRateUnder65
	for ageIndex, income := range input.IncomeStreamThousands {
		if 65 <= ageIndex+int(input.StartAge) {
			currentRoyaltyRate = input.RoyaltyRateOver65
		}
		output.OverallLoansTaken += input.AnnualLoanAmount
		output.EndingBalanceWithInterest = output.EndingBalanceWithInterest*(1+input.InterestRate/100) + input.AnnualLoanAmount
		currentAnnualIncome := income * 1000
		currentRoyalty := currentRoyaltyRate * currentAnnualIncome / 100
		currentClawback := 0.0
		if input.ClawbackBalanceTrigger <= output.EndingBalanceWithInterest {
			currentClawback = currentRoyaltyRate * input.AnnualLoanAmount / 100
		}
		output.RepaymentsFromIncome += currentRoyalty
		output.RepaymentsFromBenefitsClawbacks += currentClawback
		output.EndingBalanceWithInterest = output.EndingBalanceWithInterest - currentRoyalty - currentClawback
	}
	output.OverallLoansTaken = math.Round(output.OverallLoansTaken / 1000)
	output.RepaymentsFromIncome = math.Round(output.RepaymentsFromIncome / 1000)
	output.RepaymentsFromBenefitsClawbacks = math.Round(output.RepaymentsFromBenefitsClawbacks / 1000)
	output.EndingBalanceWithInterest = math.Round(output.EndingBalanceWithInterest / 1000)
	return
}
