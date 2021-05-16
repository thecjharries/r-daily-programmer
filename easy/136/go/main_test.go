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

func (s *MainSuite) TestStudentGetAverageFraction(c *C) {
	student := Student{
		Name:   "ABIGAIL",
		Grades: []float64{11, 3, 5, 20, 4, 2, 8, 17, 4, 5},
	}
	sum, total := student.GetAverageFraction()
	c.Assert(sum, Equals, float64(79))
	c.Assert(total, Equals, float64(10))
}

func (s *MainSuite) TestStudentGetAverage(c *C) {
	student := Student{
		Name:   "ABIGAIL",
		Grades: []float64{11, 3, 5, 20, 4, 2, 8, 17, 4, 5},
	}
	c.Assert(student.GetAverage(), Equals, 7.90)
}

func (s *MainSuite) TestStudentString(c *C) {
	student := Student{
		Name:   "ABIGAIL",
		Grades: []float64{11, 3, 5, 20, 4, 2, 8, 17, 4, 5},
	}
	c.Assert(student.String(), Equals, "ABIGAIL 7.90")
}

func (s *MainSuite) TestNewStudent(c *C) {
	name := "ABIGAIL"
	grades := []float64{11, 3, 5, 20, 4, 2, 8, 17, 4, 5}
	student := &Student{
		Name:   "ABIGAIL",
		Grades: []float64{11, 3, 5, 20, 4, 2, 8, 17, 4, 5},
	}
	c.Assert(NewStudent(name, grades), DeepEquals, student)
}

func (s *MainSuite) TestClassGetAverage(c *C) {
	class := Class{
		{
			Name:   "ABIGAIL",
			Grades: []float64{11, 3, 5, 20, 4, 2, 8, 17, 4, 5},
		},
	}
	c.Assert(class.GetAverage(), Equals, 7.90)
}

func (s *MainSuite) TestClassString(c *C) {
	class := Class{
		{
			Name:   "ABIGAIL",
			Grades: []float64{11, 3, 5, 20, 4, 2, 8, 17, 4, 5},
		},
	}
	output := `7.90
ABIGAIL 7.90
`
	c.Assert(class.String(), Equals, output)
}
