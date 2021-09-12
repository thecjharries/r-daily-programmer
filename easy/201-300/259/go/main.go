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

var keyCoordinates = map[string][2]float64{
	"1": {0, 0},
	"2": {1, 0},
	"3": {2, 0},
	"4": {0, 1},
	"5": {1, 1},
	"6": {2, 1},
	"7": {0, 2},
	"8": {1, 2},
	"9": {2, 2},
	".": {0, 3},
	"0": {1, 3},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func determineIpDistance(ip string) (distance float64) {
	for index := 1; index < len(ip); index++ {
		previous, _ := keyCoordinates[ip[index-1:index]]
		current, _ := keyCoordinates[ip[index:index+1]]
		distance += math.Sqrt(math.Pow(current[0]-previous[0], 2) + math.Pow(current[1]-previous[1], 2))
	}
	return math.Round(distance*100) / 100
}
