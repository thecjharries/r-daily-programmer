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
	male, female := createRabbits()
	male[2] = initialMale
	female[2] = initialFemale
	for !haveRabbitsTakenOver(male, female, worldDomination) {
		monthsNeeded++
		newMales := 0
		newFemales := 0
		for index := 4; index < len(female); index++ {
			newMales += female[index] * 5
			newFemales += female[index] * 9
		}
		for index := len(male) - 2; index >= 0; index-- {
			male[index+1] = male[index]
			female[index+1] = female[index]
		}
		male[0] = newMales
		female[0] = newFemales
	}
	return
}
