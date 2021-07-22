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

func (s *MainSuite) TestGetSortedValuesAndMap(c *C) {
	input := map[string]float64{
		"bholzer":      101.09,
		"Cosmologicon": 27.45,
		"nint22":       13.76,
		"nooodl":       7.29,
		"nottoobadguy": 74.56,
		"oskar_s":      39.90,
		"Steve132":     61.82,
	}
	desiredValues := []float64{7.29, 13.76, 27.45, 39.9, 61.82, 74.56, 101.09}
	desiredMap := map[float64]string{
		101.09: "bholzer",
		27.45:  "Cosmologicon",
		13.76:  "nint22",
		7.29:   "nooodl",
		74.56:  "nottoobadguy",
		39.90:  "oskar_s",
		61.82:  "Steve132",
	}
	foundValues, foundMap := getSortedValuesAndMap(input)
	c.Assert(foundValues, DeepEquals, desiredValues)
	c.Assert(foundMap, DeepEquals, desiredMap)
}

func (s *MainSuite) TestBuildFlair(c *C) {
	input := map[string]float64{
		"bholzer":      101.09,
		"Cosmologicon": 27.45,
		"nint22":       13.76,
		"nooodl":       7.29,
		"nottoobadguy": 74.56,
		"oskar_s":      39.90,
		"Steve132":     61.82,
	}
	output := "nooodl: 52\nnint22: 53\nCosmologicon: 46\noskar_s: 47\nSteve132: 38\nnottoobadguy: 47\nbholzer: 33\n"
	c.Assert(buildFlair(input), Equals, output)
}
