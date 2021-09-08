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

func (s *MainSuite) TestParseYears(c *C) {
	c.Assert(parseYears(), DeepEquals, [][]int{[]int{1732, 1799}, []int{1735, 1826}, []int{1743, 1826}, []int{1751, 1836}, []int{1758, 1831}, []int{1767, 1848}, []int{1767, 1845}, []int{1782, 1862}, []int{1773, 1841}, []int{1790, 1862}, []int{1795, 1849}, []int{1784, 1850}, []int{1800, 1874}, []int{1804, 1869}, []int{1791, 1868}, []int{1809, 1865}, []int{1808, 1875}, []int{1822, 1885}, []int{1822, 1893}, []int{1831, 1881}, []int{1829, 1886}, []int{1837, 1908}, []int{1833, 1901}, []int{1843, 1901}, []int{1858, 1919}, []int{1857, 1930}, []int{1856, 1924}, []int{1865, 1923}, []int{1872, 1933}, []int{1874, 1964}, []int{1882, 1945}, []int{1884, 1972}, []int{1890, 1969}, []int{1917, 1963}, []int{1908, 1973}, []int{1913, 1994}, []int{1913, 2006}, []int{1924, 2016}, []int{1911, 2004}, []int{1924, 2016}, []int{1946, 2016}, []int{1946, 2016}, []int{1961, 2016}})
}
