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
	"math"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

// https://old.reddit.com/r/dailyprogrammer/comments/6coqwk/20170522_challenge_316_easy_knights_metric/dhw8sdx/
func determineNeededNumberOfKnightMoves(goalX, goalY int) (result int) {
	x, y := goalX, goalY
	if x > y {
		x, y = y, x
	}
	additionalMove := 0.0
	if 0 != (x+y)%3 {
		additionalMove = 1.0
		if 0 == (x+2+y-1)%3 {
			x += 2
			y -= 1
		} else {
			x += 1
			y -= 2
		}
	}
	return int(math.Abs(math.Floor(float64(2*y-x)/3)) + math.Abs(math.Floor(float64(2*x-y)/3)+additionalMove))
}
