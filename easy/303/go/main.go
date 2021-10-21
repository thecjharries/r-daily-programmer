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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

// https://old.reddit.com/r/dailyprogrammer/comments/5vb1wf/20170221_challenge_303_easy_ricochet/de2ia2f/
func calculateRicochet(gridHeight, gridWidth, velocity int) (output string) {
	path := gridHeight * gridWidth / gcd(gridHeight, gridWidth)
	time := path / gcd(velocity, path)
	numberOfRounds := velocity * time / path
	vertical := "U"
	if 1 == (velocity*time/gridHeight)%2 {
		vertical = "L"
	}
	horizontal := "L"
	if 1 == (velocity*time/gridWidth)%2 {
		horizontal = "R"
	}
	numberOfBounces := (path/gridHeight+path/gridWidth-1)*numberOfRounds - 1
	return fmt.Sprintf("%s%s %d %d", vertical, horizontal, numberOfBounces, time)
}
