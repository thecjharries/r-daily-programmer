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
	"math/rand"
)

var operators = []string{"+", "-", "*", "/"}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func buildEquation(minInt, maxInt int) string {
	randomInts := make([]int, 4)
	randomOps := make([]string, 3)
	for index := 0; index < 3; index++ {
		randomInts[index] = rand.Intn(maxInt-minInt) + minInt + 1
		randomOps[index] = operators[rand.Intn(len(operators))]
	}
	randomInts[3] = rand.Intn(maxInt-minInt) + minInt + 1
	return fmt.Sprintf(
		"%d %s %d %s %d %s %d",
		randomInts[0],
		randomOps[0],
		randomInts[1],
		randomOps[1],
		randomInts[2],
		randomOps[2],
		randomInts[3],
	)
}
