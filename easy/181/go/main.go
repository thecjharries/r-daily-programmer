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
	"regexp"
	"strconv"
)

var equationPattern = regexp.MustCompile(`y=(?:(.+)x)?\+?(.+)?`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseSingleEquation(equation string) (a, b float64) {
	match := equationPattern.FindAllStringSubmatch(equation, -1)[0]
	a, _ = strconv.ParseFloat(match[1], 64)
	b, _ = strconv.ParseFloat(match[2], 64)
	return
}

//
//func parseEquations(first, second string) (a1, b1, a2, b2 float64) {
//
//	return
//}
