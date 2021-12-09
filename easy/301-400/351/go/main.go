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
	"strconv"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

// https://old.reddit.com/r/dailyprogrammer/comments/7x81yg/20180213_challenge_351_easy_cricket_scoring/du6ezb6/
func parseCricketScore(input string) (scores []int) {
	extrasIndex := 0
	activeBatterIndex := 1
	nextBatterIndex := 2
	scores = make([]int, 12)
	over := 0
	wickets := 0
	nextAtBatIndex := 3
	for _, score := range input {
		legalBall := true
		number, err := strconv.Atoi(string(score))
		if nil == err {
			scores[activeBatterIndex] = scores[activeBatterIndex] + number
			if 1 == number%2 {
				activeBatterIndex, nextBatterIndex = nextBatterIndex, activeBatterIndex
			}
		} else {
			if 'b' == score {
				scores[extrasIndex] = scores[extrasIndex] + 1
				activeBatterIndex, nextBatterIndex = nextBatterIndex, activeBatterIndex
			} else if 'w' == score {
				scores[extrasIndex] = scores[extrasIndex] + 1
				legalBall = false
			} else if 'W' == score {
				wickets++
				if 10 == wickets {
					break
				}
				activeBatterIndex = nextAtBatIndex
				nextAtBatIndex++
			}
		}
		if legalBall {
			over++
		}
		if 6 == over {
			over = 0
			activeBatterIndex, nextBatterIndex = nextBatterIndex, activeBatterIndex
		}
		fmt.Println(scores)
	}
	return
}
