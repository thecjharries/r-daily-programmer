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

func findKaprekarNumbersInRange(start, end int) (numbers []int) {
	if 1 == start {
		numbers = append(numbers, start)
	}
	for possibleKaprekarNumber := start; possibleKaprekarNumber <= end; possibleKaprekarNumber++ {
		squareString := strconv.Itoa(possibleKaprekarNumber * possibleKaprekarNumber)
		for index := 1; index < len(squareString); index++ {
			first, _ := strconv.Atoi(squareString[:index])
			second, _ := strconv.Atoi(squareString[index:])
			if 0 < second && possibleKaprekarNumber == first+second {
				numbers = append(numbers, possibleKaprekarNumber)
				break
			}
		}
	}
	return
}
