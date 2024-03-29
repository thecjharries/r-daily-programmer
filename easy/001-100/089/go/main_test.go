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
	c.Assert(printSpyContents, Equals, "Mean is 0.33\nVariance is 0.07\nStandard deviation is 0.26\n")
}

func (s *MainSuite) TestLoadData(c *C) {
	data := loadData()
	c.Assert(len(data), Equals, 60)
}

func (s *MainSuite) TestArithmeticMean(c *C) {
	data := []float64{600, 470, 170, 430, 300}
	c.Assert(arithmeticMean(data), Equals, float64(394))
}

func (s *MainSuite) TestVariance(c *C) {
	data := []float64{600, 470, 170, 430, 300}
	c.Assert(variance(data), Equals, float64(21704))
}

func (s *MainSuite) TestStandardDeviation(c *C) {
	data := []float64{600, 470, 170, 430, 300}
	c.Assert(standardDeviation(data), Equals, 147.32277488562318)
}

func (s *MainSuite) TestPromptStatisticalSuite(c *C) {
	data := []float64{600, 470, 170, 430, 300}
	c.Assert(promptStatisticalSuite(data), Equals, "Mean is 394.00\nVariance is 21704.00\nStandard deviation is 147.32\n")
}
