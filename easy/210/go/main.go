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
	"strconv"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findBinaryAndOpposite(input int) (string, int) {
	binary := strconv.FormatInt(int64(input), 2)
	binary = strings.Repeat("0", 8-len(binary)) + binary
	oppositeBinary := ""
	for _, character := range binary {
		if '1' == character {
			oppositeBinary += "0"
		} else {
			oppositeBinary += "1"
		}
	}
	opposite64, _ := strconv.ParseInt(oppositeBinary, 2, 0)
	return binary, int(opposite64)
}

func findCompabilityPercent(first, second string) float64 {
	count := 0.0
	for index := 0; index < 8; index++ {
		if first[index] == second[index] {
			count += 1
		}
	}
	return math.Round(count*100) / 100
}
