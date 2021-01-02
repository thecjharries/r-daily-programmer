// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

package main

import (
	"strings"
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {
	getInputReader *strings.Reader
}

const getStringInputInput string = "test\n"
const getStringInputOutput string = "test"

type getFloat64InputFixture struct {
	input string
	result float64
}

var getFloat64InputFixtures = []getFloat64InputFixture{
	{"0", 0},
	{"4.2", 4.2},
	{"150.64", 150.64},
}

type float64InputResultFixture struct {
	input float64
	result float64
}

var roundToUsdFixtures = []float64InputResultFixture{
	{0, 0.00},
	{1.1, 1.10},
	{2.22, 2.22},
	{3.333, 3.33},
	{4.4444, 4.44},
	{5.55555, 5.56},
}

var getUsdPercentageFrom100Fixtures = []float64InputResultFixture{
	{0, 0.00},
	{5, 5.00},
	{10, 10.00},
	{20.5, 20.50},
}

var _ = Suite(&MainSuite{
	getInputReader: strings.NewReader(getStringInputInput),
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGetStringInput(c *C) {
	input := getStringInput("", s.getInputReader)
	c.Assert(input, Equals, getStringInputOutput)
}

func (s *MainSuite) TestGetFloat64Input(c *C) {
	for _, fixture := range getFloat64InputFixtures {
		input := getFloat64Input("", strings.NewReader(fixture.input))
		c.Assert(input, Equals, fixture.result)
	}
}

func (s *MainSuite) TestRoundToUsd(c *C) {
	for _, fixture := range roundToUsdFixtures {
		result := roundToUsd(fixture.input)
		c.Assert(result, Equals, fixture.result)
	}
}

func (s *MainSuite) TestGetUsdPercentage(c *C) {
	for _, fixture := range getUsdPercentageFrom100Fixtures {
		result := getUsdPercentage(100, fixture.input)
		c.Assert(result, Equals, fixture.result)
	}
}
