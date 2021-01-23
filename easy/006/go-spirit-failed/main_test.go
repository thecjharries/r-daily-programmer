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
	"github.com/shopspring/decimal"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {
	roundPrecision int32
	number decimal.Decimal
}

var _ = Suite(&MainSuite{
	roundPrecision: 2,
	// The result should be 2/3 so we can test rounding
	number: decimal.New(2, 0).Div(decimal.New(3, 0)),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestRoundDecimalToPrecision(c *C) {
	c.Assert(roundDecimalToPrecision(s.number, s.roundPrecision).StringFixed(s.roundPrecision), Equals, "0.67")
}

func (s *MainSuite) TestLeibnizConvergenceFormulaToNPlaces(c *C) {
	c.Assert(leibnizConvergenceFormulaNthTerm(0), DeepEquals, twoDecimal.Div(threeDecimal))
	c.Assert(leibnizConvergenceFormulaNthTerm(1), DeepEquals, twoDecimal.Div(decimal.New(35, 0)))
}
