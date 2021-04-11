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

func main() {
	input := int64(23)
	inputBinary := convertToBinary(input)
	populationCount := countBinaryPopulationCount(inputBinary)
	fmt.Printf("%d has a population count of %d\n", input, populationCount)
}

func convertToBinary(input int64) string {
	return strconv.FormatInt(input, 2)
}

func countBinaryPopulationCount(binary string) int {
	count := 0
	exploded := strings.Split(binary, "")
	for _, character := range exploded {
		positionCount, _ := strconv.Atoi(character)
		count += positionCount
	}
	return count
}
