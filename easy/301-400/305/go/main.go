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

func convertIntToPermBase2(input int) string {
	inputAsFloat64 := float64(input)
	power := 1.0
	for math.Pow(2, power) <= inputAsFloat64 {
		inputAsFloat64 -= math.Pow(2, power)
		power += 1
	}
	fmt.Println(power)
	return fmt.Sprintf(fmt.Sprintf("%%0%db", int64(power)), int64(inputAsFloat64))
}

func convertPermBase2ToInt(input string) (output int) {
	fromBinary, _ := strconv.ParseInt(input, 2, 0)
	return int(fromBinary) + int(math.Pow(2, float64(len(input)))) - 2
}
