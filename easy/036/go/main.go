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
	lockerCount := 1000
	lockers := openLockers(lockerCount)
	for lockerNumber, isOpen := range lockers {
		if isOpen {
			fmt.Println(lockerNumber + 1)
		}
	}
}

// Runs the open locker problem for given number of lockers
func openLockers(lockerCount int) (lockers []bool) {
	lockers = make([]bool, lockerCount)
	for studentNumber := 1; studentNumber <= lockerCount; studentNumber++ {
		for lockerNumber := studentNumber; lockerNumber <= lockerCount; lockerNumber += studentNumber {
			lockers[lockerNumber - 1] = !lockers[lockerNumber - 1]
		}
	}
	return
}
