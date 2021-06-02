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

func discoverTornNumbers() (tornNumbers []int) {
	for lower := 1; lower < 100; lower++ {
		for upper := 10; upper < 100; upper++ {
			potentialNumber := 100*upper + lower
			if (upper+lower)*(upper+lower) == potentialNumber {
				potentialNumberString := strconv.Itoa(potentialNumber)
				unique := make(map[rune]bool)
				for _, element := range potentialNumberString {
					unique[element] = true
				}
				if 4 == len(unique) {
					tornNumbers = append(tornNumbers, potentialNumber)
				}
			}
		}
	}
	return
}
