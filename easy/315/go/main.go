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
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func xorMultiplication(first, second int) (result int) {
	firstBinary := strconv.FormatInt(int64(first), 2)
	secondBinary := strconv.FormatInt(int64(second), 2)
	for index := len(secondBinary) - 1; index >= 0; index-- {
		if "1" == secondBinary[index:index+1] {
			current, _ := strconv.ParseInt(firstBinary, 2, 0)
			result ^= int(current)
		}
		firstBinary = firstBinary + "0"
	}
	return
}
