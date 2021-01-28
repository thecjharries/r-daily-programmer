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

// Determines whether or not the given year is a leap year
// https://en.wikipedia.org/wiki/Leap_year#Algorithm
func isLeapYear(year int) bool {
	if 0 != year % 4 {
		return false
	} else if 0 != year % 100 {
		return true
	} else if 0 != year % 400 {
		return false
	}
	return true
}
