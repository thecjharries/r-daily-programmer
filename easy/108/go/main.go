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
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func getExponent(number float64) (exponent float64) {
	return math.Floor(math.Log10(number))
}

func convertToScientific(number float64) string {
	exponent := getExponent(number)
	return fmt.Sprintf(
		"%s x 10^%d",
		strconv.FormatFloat(number/math.Pow(10, exponent), 'f', -1, 64),
		int64(exponent),
	)
}
