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

func (s *MainSuite) TestNewAccount(c *C) {
	var account Account
	account = NewAccount(10, "test")
	c.Assert(account.Number, Equals, 10)
	c.Assert(account.Name, Equals, "test")
	c.Assert(account.Balance, Equals, 0.0)
	account = NewAccount(20, 14.64)
	c.Assert(account.Number, Equals, 20)
	c.Assert(account.Name, Equals, "")
	c.Assert(account.Balance, Equals, 14.64)
	account = NewAccount(30, 5)
	c.Assert(account.Number, Equals, 30)
	c.Assert(account.Name, Equals, "")
	c.Assert(account.Balance, Equals, 0.0)
}

func (s *MainSuite) TestNewJournal(c *C) {
	journal := NewJournal(
		"1000;JAN-16;100000;0;\n3000;JAN-16;0;100000;\n7140;JAN-16;36000;0;\n1000;JAN-16;0;36000;\n1100;FEB-16;80000;0;\n1000;FEB-16;0;60000;\n2000;FEB-16;0;20000;\n1110;FEB-16;17600;0;\n2010;FEB-16;0;17600;\n1000;MAR-16;28500;0;\n4000;MAR-16;0;28500;\n2010;MAR-16;17600;0;\n1000;MAR-16;0;17600;\n5000;APR-16;19100;0;\n1000;APR-16;0;19100;\n1000;APR-16;32900;0;\n1020;APR-16;21200;0;\n4000;APR-16;0;54100;\n1000;MAY-16;15300;0;\n1020;MAY-16;0;15300;\n1000;MAY-16;4000;0;\n4090;MAY-16;0;4000;\n1110;JUN-16;5200;0;\n2010;JUN-16;0;5200;\n5100;JUN-16;19100;0;\n1000;JUN-16;0;19100;\n4120;JUN-16;5000;0;\n1000;JUN-16;0;5000;\n7160;JUL-16;2470;0;\n2010;JUL-16;0;2470;\n5500;JUL-16;3470;0;\n1000;JUL-16;0;3470;",
		"1000;Cash;\n1020;Account Receivables;\n1100;Lab Equipement;\n1110;Office Supplies;\n2000;Notes Payables;\n2010;Account Payables;\n2110;Utilities Payables;\n3000;Common Stock;\n4000;Commercial Revenue;\n4090;Unearned Revenue;\n5000;Direct Labor;\n5100;Consultants;\n5500;Misc Costs;\n7140;Rent;\n7160;Telephone;\n9090;Dividends;",
	)
	c.Assert(journal[1000].Balance, Equals, 20430.0)
}
