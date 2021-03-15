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

func main() {
	fmt.Println("hello world")
}

func generateFibonacciSequenceLessThan(input int64, fibonacciSequence []int64) []int64 {
	if 0 == len(fibonacciSequence) {
		return generateFibonacciSequenceLessThan(input, []int64{0, 1})
	}
	penultimate := fibonacciSequence[len(fibonacciSequence) - 2]
	ultimate := fibonacciSequence[len(fibonacciSequence) - 1]
	if input > penultimate + ultimate {
		return generateFibonacciSequenceLessThan(input, append(fibonacciSequence, penultimate + ultimate))
	}
	return fibonacciSequence
}
