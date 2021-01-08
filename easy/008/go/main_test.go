// Copyright 2020 CJ Harries
// Licensed under http://www.apache.org/licenses/LICENSE-2.0

package main

import (
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {
	givenBottleCount int
	givenGeneratedLyric string
}

var _ = Suite(&MainSuite{
	givenBottleCount: 2,
	givenGeneratedLyric: `2 bottles of beer on the wall, 2 bottles of beer.
Take one down, pass it around, 1 bottles of beer on the wall...
`,
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGenerateLyrics(c *C) {
	c.Assert(
		func() {
			generateLyrics(0)
		},
		Panics,
		errorBadBottleCount,
	)
	c.Assert(generateLyrics(1), Equals, lastLyric)
	c.Assert(generateLyrics(s.givenBottleCount), Equals, s.givenGeneratedLyric)
}
