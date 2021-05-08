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
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func hashStringIntBySummingDigits(input string) string {
	exploded := strings.Split(input, "")
	sum := 0
	for _, digit := range exploded {
		converted, _ := strconv.Atoi(digit)
		sum += converted
	}
	return strconv.Itoa(sum)
}

func generateStepsToPositiveHashUnder10(input string) (steps []string) {
	steps = append(steps, input)
	for 1 < len(steps[len(steps)-1]) {
		steps = append(steps, hashStringIntBySummingDigits(steps[len(steps)-1]))
	}
	return
}
