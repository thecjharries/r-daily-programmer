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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func isPrime(input int) bool {
	if input < 2 {
		return false
	} else if 2 == input {
		return true
	} else if 0 == input%2 {
		return false
	} else if 5 < input && 0 == input%5 {
		return false
	} else {
		for index := 3; index < int(math.Sqrt(float64(input)))+1; index += 2 {
			if 0 == input%index {
				return false
			}
		}
	}
	return true
}
