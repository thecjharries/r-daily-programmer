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
	iterations := 100
	_, _ = zPrint(calculateDottieNumber(iterations))
}

func calculateDottieNumber(iterations int) (dottie float64) {
	dottie = 0
	for index := 0; index < iterations; index++ {
		dottie = math.Cos(dottie)
	}
	return
}
