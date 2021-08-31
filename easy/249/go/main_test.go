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

func (s *MainSuite) TestFindBestPrices(c *C) {
	var prices []float64
	var low, high float64
	prices = []float64{19.35, 19.30, 18.88, 18.93, 18.95, 19.03, 19.00, 18.97, 18.97, 18.98}
	low, high = findBestPrices(prices)
	c.Assert(low, Equals, 18.88)
	c.Assert(high, Equals, 19.03)
	prices = []float64{9.20, 8.03, 10.02, 8.08, 8.14, 8.10, 8.31, 8.28, 8.35, 8.34, 8.39, 8.45, 8.38, 8.38, 8.32, 8.36, 8.28, 8.28, 8.38, 8.48, 8.49, 8.54, 8.73, 8.72, 8.76, 8.74, 8.87, 8.82, 8.81, 8.82, 8.85, 8.85, 8.86, 8.63, 8.70, 8.68, 8.72, 8.77, 8.69, 8.65, 8.70, 8.98, 8.98, 8.87, 8.71, 9.17, 9.34, 9.28, 8.98, 9.02, 9.16, 9.15, 9.07, 9.14, 9.13, 9.10, 9.16, 9.06, 9.10, 9.15, 9.11, 8.72, 8.86, 8.83, 8.70, 8.69, 8.73, 8.73, 8.67, 8.70, 8.69, 8.81, 8.82, 8.83, 8.91, 8.80, 8.97, 8.86, 8.81, 8.87, 8.82, 8.78, 8.82, 8.77, 8.54, 8.32, 8.33, 8.32, 8.51, 8.53, 8.52, 8.41, 8.55, 8.31, 8.38, 8.34, 8.34, 8.19, 8.17, 8.16}
	low, high = findBestPrices(prices)
	c.Assert(low, Equals, 8.03)
	c.Assert(high, Equals, 9.34)
}
