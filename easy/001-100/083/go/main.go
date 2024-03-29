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

type NumberScale struct {
	Number uint64
	Short  string
	Long   string
}

var ScalesLargestToSmallest = []NumberScale{
	{1000000000000000000, "quintillion", "trillion"},
	{1000000000000000, "quadrillion", "billiard"},
	{1000000000000, "trillion", "billion"},
	{1000000000, "billion", "milliard"},
	{1000000, "million", "million"},
	{1000, "thousand", "thousand"},
	{1, "", ""},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertToScaleRepresentation(number uint64) (representation []int) {
	representation = make([]int, len(ScalesLargestToSmallest))
	currentNumber := number
	for index, scale := range ScalesLargestToSmallest {
		if scale.Number < currentNumber {
			factor := currentNumber / scale.Number
			representation[index] = int(factor)
			currentNumber -= factor * scale.Number
		}
	}
	return
}

func buildScaleRepresentationStrings(scaleRepresentation []int) (short, long string) {
	comma := ""
	nonZeroComponents := 0
	finalIndex := len(ScalesLargestToSmallest) - 1
	for index, scale := range ScalesLargestToSmallest[:finalIndex] {
		if 0 < scaleRepresentation[index] {
			short += fmt.Sprintf("%s%d %s", comma, scaleRepresentation[index], scale.Short)
			long += fmt.Sprintf("%s%d %s", comma, scaleRepresentation[index], scale.Long)
			comma = ", "
			nonZeroComponents++
		}
	}
	if 0 < scaleRepresentation[finalIndex] {
		if 1 == nonZeroComponents {
			comma = " and "
		} else if 1 < nonZeroComponents {
			comma += "and "
		}
		short += fmt.Sprintf("%s%d", comma, scaleRepresentation[finalIndex])
		long += fmt.Sprintf("%s%d", comma, scaleRepresentation[finalIndex])
	}
	return
}
