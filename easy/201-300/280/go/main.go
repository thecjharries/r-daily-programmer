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
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertHandCountingToSum(input string) int {
	ones, tens := "", ""
	for index := 0; index < 5; index++ {
		ones += input[len(input)-1-index : len(input)-index]
		tens += input[index : index+1]
	}
	ones = strings.TrimLeft(ones, "0")
	tens = strings.TrimLeft(tens, "0")
	if strings.Contains(ones[:len(ones)-1], "0") || strings.Contains(tens[:len(tens)-1], "0") {
		return -1
	}
	sum := strings.Count(ones, "1")
	if "1" == ones[len(ones)-1:] {
		sum += 4
	}
	sum += strings.Count(tens, "1") * 10
	if "1" == tens[len(tens)-1:] {
		sum += 40
	}
	return sum
}
