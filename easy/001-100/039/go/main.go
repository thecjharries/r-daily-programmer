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

func main() {
	max := 15
	output := fizzBuzz(max)
	for _, line := range output {
		fmt.Println(line)
	}
}

// Performs (slightly abnormal) fizzbuzz
func fizzBuzz(max int) (output []string) {
	output = make([]string, max)
	for index := 1; index <= max; index++ {
		if 0 == index%3 {
			output[index-1] += "Fizz"
		}
		if 0 == index%5 {
			output[index-1] += "Buzz"
		}
		if 0 == len(output[index-1]) {
			output[index-1] = strconv.Itoa(index)
		}
	}
	return
}
