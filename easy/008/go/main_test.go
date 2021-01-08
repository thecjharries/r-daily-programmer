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
	givenGeneratedSong string
}

var _ = Suite(&MainSuite{
	givenBottleCount: 2,
	givenGeneratedLyric: `2 bottles of beer on the wall, 2 bottles of beer.
Take one down, pass it around, 1 bottles of beer on the wall...
`,
	givenGeneratedSong: `2 bottles of beer on the wall, 2 bottles of beer.
Take one down, pass it around, 1 bottles of beer on the wall...
1 bottles of beer on the wall, 1 bottles of beer.
Take one down, pass it around, 0 bottles of beer on the wall...
No more bottles of beer on the wall, no more bottles of beer.
We've taken them down and passed them around; now we're drunk and passed out!
`,
})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestGenerateLyrics(c *C) {
	c.Assert(
		func() {
			generateLyrics(-1)
		},
		Panics,
		errorBadBottleCount,
	)
	c.Assert(generateLyrics(0), Equals, lastLyric)
	c.Assert(generateLyrics(s.givenBottleCount), Equals, s.givenGeneratedLyric)
}

func (s *MainSuite) TestGenerateSong(c *C) {
	c.Assert(generateSong(s.givenBottleCount), Equals, s.givenGeneratedSong)
}
