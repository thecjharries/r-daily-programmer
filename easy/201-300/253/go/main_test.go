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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

var printCallCount int
var printSpyContents string

func printSpy(a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprint(a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Println
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestCalculateLoanResults(c *C) {
	var input LoanConsiderations
	var output LoanResults
	input = LoanConsiderations{
		InterestRate:           2,
		AnnualLoanAmount:       15000,
		StartAge:               18,
		ClawbackBalanceTrigger: 100000,
		RoyaltyRateUnder65:     20,
		RoyaltyRateOver65:      40,
		IncomeStreamThousands:  []float64{0, 0, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
	}
	output = LoanResults{
		OverallLoansTaken:               1080,
		RepaymentsFromIncome:            280,
		RepaymentsFromBenefitsClawbacks: 270,
		EndingBalanceWithInterest:       1169,
	}
	c.Assert(calculateLoanResults(input), DeepEquals, output)
	input = LoanConsiderations{
		InterestRate:           2,
		AnnualLoanAmount:       15000,
		StartAge:               18,
		ClawbackBalanceTrigger: 100000,
		RoyaltyRateUnder65:     20,
		RoyaltyRateOver65:      40,
		IncomeStreamThousands:  []float64{0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 100, 120, 140, 160, 200, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10},
	}
	output = LoanResults{
		OverallLoansTaken:               1005,
		RepaymentsFromIncome:            584,
		RepaymentsFromBenefitsClawbacks: 237,
		EndingBalanceWithInterest:       509,
	}
	c.Assert(calculateLoanResults(input), DeepEquals, output)
}
