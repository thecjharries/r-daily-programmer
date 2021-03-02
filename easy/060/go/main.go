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

func main() {
	fmt.Println("hello world")
}

func isIntPolite(number int) bool {
	testingNumber := number
	for 0 == testingNumber % 2 {
		testingNumber /= 2
	}
	return testingNumber > 1
}

func removeEvenDivisors(number int) int {
	reducingNumber := number
	for 0 == reducingNumber % 2 {
		reducingNumber /= 2
	}
	return reducingNumber
}

// https://www.geeksforgeeks.org/find-politeness-number/
func determinePolitenessOfInt(number int) int {
	politeness := 1
	testingNumber := number
	for 0 == testingNumber % 2 {
		testingNumber /= 2
	}
	oddFactor := 3
	return oddFactor * politeness
}
