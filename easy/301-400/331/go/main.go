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
)

var operators = map[string]func(a, b float64) float64{
	"+": func(a, b float64) float64 { return a + b },
	"-": func(a, b float64) float64 { return a - b },
	"*": func(a, b float64) float64 { return a * b },
	"/": func(a, b float64) float64 { return a / b },
	"^": func(a, b float64) float64 { return math.Pow(a, b) },
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func isInteger(input float64) bool {
	return input == float64(int(input))
}

func calculate(a, b int, operator string) (int, error) {
	value := operators[operator](float64(a), float64(b))
	if isInteger(value) {
		return int(value), nil
	}
	return 0, fmt.Errorf("%f is not an integer", value)
}
