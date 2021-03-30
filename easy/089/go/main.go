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
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

const dataFilename string = "./dataset.txt"

var zPrint = fmt.Println

func main() {
	_, _ = zPrint(promptStatisticalSuite(loadData()))
}

func loadData() (data []float64) {
	contents, _ := ioutil.ReadFile(dataFilename)
	exploded := strings.Split(strings.TrimSpace(string(contents)), "\n")
	data = make([]float64, len(exploded))
	for index, number := range exploded {
		data[index], _ = strconv.ParseFloat(number, 64)
	}
	return
}

func arithmeticMean(data []float64) float64 {
	totalSum := float64(0)
	for _, number := range data {
		totalSum += number
	}
	return totalSum / float64(len(data))
}

func variance(data []float64) float64 {
	mean := arithmeticMean(data)
	sum := float64(0)
	for _, number := range data {
		sum += math.Pow(number-mean, 2)
	}
	return sum / float64(len(data))
}

func standardDeviation(data []float64) float64 {
	return math.Sqrt(variance(data))
}

func promptStatisticalSuite(data []float64) string {
	dataVariance := variance(data)
	return fmt.Sprintf(
		"Mean is %0.2f\nVariance is %0.2f\nStandard deviation is %0.2f\n",
		arithmeticMean(data),
		dataVariance,
		math.Sqrt(dataVariance),
	)
}
