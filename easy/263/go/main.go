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

func calculateShannonEntropy(input string) (entropy float64) {
	frequencies := make(map[rune]float64)
	for _, character := range input {
		_, exists := frequencies[character]
		if exists {
			frequencies[character] += 1
		} else {
			frequencies[character] = 1
		}
	}
	for _, frequency := range frequencies {
		entropy += -1 * (frequency / float64(len(input))) * math.Log2(frequency/float64(len(input)))
	}
	return math.Round(entropy*1000000000) / 1000000000
}
