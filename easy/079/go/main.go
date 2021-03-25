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

import "fmt"

var zPrint = fmt.Printf

const ErrorStepsBelowTwo string = "You cannot have fewer than 2 steps"

func main() {
	_, _ = zPrint("hello world")
}

func stepCount(start, end float64, totalSteps int) (steps []float64) {
	if totalSteps < 2 {
		panic(ErrorStepsBelowTwo)
	}
	steps = append(steps, start)
	for index := 0; index < totalSteps-2; index++ {
		steps = append(steps, steps[index]+(end-start)/(float64(totalSteps)-1))
	}
	steps = append(steps, end)
	return
}
