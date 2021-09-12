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

func determinePoolSize(sailorCount float64) float64 {
	if 2 > sailorCount {
		return 0
	} else if 2 == sailorCount {
		return 11
	} else {
		if 0 == int64(sailorCount)%2 {
			return (sailorCount - 1) * (math.Pow(sailorCount, sailorCount) - 1)
		} else {
			return math.Pow(sailorCount, sailorCount) - sailorCount + 1
		}
	}
}
