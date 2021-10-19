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

func (s *MainSuite) TestPrintTwelveDays(c *C) {
	c.Assert(printTwelveDays([]string{
		"Partridge in a Pear Tree",
		"Turtle Doves",
		"French Hens",
		"Calling Birds",
		"Golden Rings",
		"Geese a Laying",
		"Swans a Swimming",
		"Maids a Milking",
		"Ladies Dancing",
		"Lords a Leaping",
		"Pipers Piping",
		"Drummers Drumming",
	}), Equals, "On the first day of Christmas\nmy true love sent to me:\n1 Partridge in a Pear Tree\n\nOn the second day of Christmas\nmy true love sent to me:\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the third day of Christmas\nmy true love sent to me:\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the fourth day of Christmas\nmy true love sent to me:\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the fifth day of Christmas\nmy true love sent to me:\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the sixth day of Christmas\nmy true love sent to me:\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the seventh day of Christmas\nmy true love sent to me:\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the eighth day of Christmas\nmy true love sent to me:\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the ninth day of Christmas\nmy true love sent to me:\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the tenth day of Christmas\nmy true love sent to me:\n10 Lords a Leaping\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the eleventh day of Christmas\nmy true love sent to me:\n11 Pipers Piping\n10 Lords a Leaping\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree\n\nOn the twelfth day of Christmas\nmy true love sent to me:\n12 Drummers Drumming\n11 Pipers Piping\n10 Lords a Leaping\n9 Ladies Dancing\n8 Maids a Milking\n7 Swans a Swimming\n6 Geese a Laying\n5 Golden Rings\n4 Calling Birds\n3 French Hens\n2 Turtle Doves\nand 1 Partridge in a Pear Tree")
}
