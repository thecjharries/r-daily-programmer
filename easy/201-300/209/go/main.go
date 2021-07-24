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
	"sort"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func getSortedValuesAndMap(input map[string]float64) (values []float64, valueKeymap map[float64]string) {
	valueKeymap = make(map[float64]string)
	for key, value := range input {
		values = append(values, value)
		valueKeymap[value] = key
	}
	sort.Float64s(values)
	return
}

func buildFlair(times map[string]float64) (output string) {
	values, valueKeyMap := getSortedValuesAndMap(times)
	previousValue := 0.0
	for _, value := range values {
		output += fmt.Sprintf("%s: %d\n", valueKeyMap[value], int(math.Floor(60-(value-previousValue))))
		previousValue = value
	}
	return
}
