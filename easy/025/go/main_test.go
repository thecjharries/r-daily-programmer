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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestVoteCountTotalVotes(c *C) {
	sample := VoteCount{
		"one": 1,
		"two": 2,
		"three": 3,
	}
	c.Assert(sample.TotalVotes(), Equals, 6)
}

func (s *MainSuite) TestVoteCountCandidateWithMostVotes(c *C) {
	sample := VoteCount{
		"one": 1,
		"two": 2,
		"three": 3,
	}
	candidateWithMostVotes, candidateVotes, totalVotes := sample.CandidateWithMostVotes()
	c.Assert(candidateWithMostVotes, Equals, "three")
	c.Assert(candidateVotes, Equals, 3)
	c.Assert(totalVotes, Equals, 6)
}


func (s *MainSuite) TestVoteCountMajorityWinnerWithNoWinner(c *C) {
	sample := VoteCount{
		"one": 1,
		"two": 2,
		"three": 3,
	}
	c.Assert(sample.MajorityWinner(), Equals, "")
}

func (s *MainSuite) TestVoteCountMajorityWinnerWithWinner(c *C) {
	sample := VoteCount{
		"one": 1,
		"two": 2,
		"four": 4,
	}
	c.Assert(sample.MajorityWinner(), Equals, "four")
}
