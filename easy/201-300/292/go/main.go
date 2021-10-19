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

func buildRange(input string) (output string) {
	explodedInput := strings.Split(input, ",")
	outputNumbers := make([]string, 0)
	for _, chunk := range explodedInput {
		_, parseErr := strconv.Atoi(chunk)
		if nil == parseErr {
			if 0 == len(outputNumbers) {
				outputNumbers = append(outputNumbers, chunk)
				continue
			}
			index, _ := strconv.Atoi(outputNumbers[len(outputNumbers)-1])
			for !strings.HasSuffix(strconv.Itoa(index), chunk) {
				index++
			}
			outputNumbers = append(outputNumbers, strconv.Itoa(index))
		} else {
			if strings.Contains(chunk, "-") {
				rangeExploded := strings.Split(chunk, "-")
				var start int
				if 0 == len(outputNumbers) {
					outputNumbers = append(outputNumbers, rangeExploded[0])
					start, _ = strconv.Atoi(rangeExploded[0])
					start++
				} else {
					start, _ = strconv.Atoi(outputNumbers[len(outputNumbers)-1])
					for !strings.HasSuffix(strconv.Itoa(start), rangeExploded[0]) {
						start++
					}
				}
				index := start
				for !strings.HasSuffix(strconv.Itoa(index), rangeExploded[1]) {
					outputNumbers = append(outputNumbers, strconv.Itoa(index))
					index++
				}
				outputNumbers = append(outputNumbers, strconv.Itoa(index))
			} else if strings.Contains(chunk, ":") {
				rangeExploded := strings.Split(chunk, ":")
				increment, _ := strconv.Atoi(rangeExploded[2])
				var start int
				if 0 == len(outputNumbers) {
					outputNumbers = append(outputNumbers, rangeExploded[0])
					start, _ = strconv.Atoi(rangeExploded[0])
					start += increment
				} else {
					start, _ = strconv.Atoi(outputNumbers[len(outputNumbers)-1])
					for !strings.HasSuffix(strconv.Itoa(start), rangeExploded[0]) {
						start++
					}
				}
				index := start
				for !strings.HasSuffix(strconv.Itoa(index), rangeExploded[1]) {
					outputNumbers = append(outputNumbers, strconv.Itoa(index))

					index += increment
				}
				outputNumbers = append(outputNumbers, strconv.Itoa(index))
			}
		}
	}
	return strings.Join(outputNumbers, " ")
}
