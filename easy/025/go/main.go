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

import "fmt"

type VoteCount map[string]int

// Sum all the votes in an vote
func (v *VoteCount) TotalVotes() (totalVotes int) {
	for _, vote := range *v {
		totalVotes += vote
	}
	return
}

// Find candidate with most votes
func (v *VoteCount) CandidateWithMostVotes() (candidateWithMostVotes string, candidateVotes, totalVotes int) {
	candidateVotes = -1
	for candidate, voteCount := range *v {
		totalVotes += voteCount
		if candidateVotes < voteCount {
			candidateWithMostVotes = candidate
			candidateVotes = voteCount
		}
	}
	return
}

// Determine whether or not there is a majority winner
// Majority is defined as more than half, ie 'winnerCount > total / 2'
// If empty, no winner
func (v *VoteCount) MajorityWinner() (winner string) {
	candidateWithMostVotes, candidateVotes, totalVotes := v.CandidateWithMostVotes()
	if float64(totalVotes) / 2 < float64(candidateVotes) {
		return candidateWithMostVotes
	}
	return
}

func main() {
	fmt.Println("hello world")
}
