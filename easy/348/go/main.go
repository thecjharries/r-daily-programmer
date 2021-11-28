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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func createRabbits() (male, female map[int]int) {
	male = make(map[int]int)
	female = make(map[int]int)
	for index := 0; index < 96; index++ {
		male[index] = 0
		female[index] = 0
	}
	return
}

func haveRabbitsTakenOver(male, female map[int]int, maxRabbits int) bool {
	totalRabbits := 0
	for index := range male {
		totalRabbits += male[index] + female[index]
	}
	return maxRabbits <= totalRabbits
}

func determineMonthsNeededToTakeOver(initialMale, initialFemale, worldDomination int) (monthsNeeded int) {
	return
}
