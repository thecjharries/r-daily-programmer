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
	"math/rand"
	"time"
)

var doors = []int{1, 2, 3}

var zSeed = time.Now().UnixNano()

const maxRounds = 10000

func main() {
	rand.Seed(zSeed)
	var winSwitch, winNoSwitch int
	for index := 0; index < maxRounds; index++ {
		if gameRound(true) {
			winSwitch++
		}
		if gameRound(false) {
			winNoSwitch++
		}
	}
	fmt.Printf("Win With Switch %%: %f\n", float64(winSwitch*100)/float64(maxRounds))
	fmt.Printf("Win Without Switch %%: %f\n", float64(winNoSwitch*100)/float64(maxRounds))
}

func pickRandomIndex() int {
	return rand.Intn(len(doors))
}

func pickDoorIndexToRemove(winningIndex, playerIndex int) int {
	removedDoor := winningIndex
	for removedDoor == winningIndex || removedDoor == playerIndex {
		removedDoor = pickRandomIndex()
	}
	return removedDoor
}

func gameRound(playerSwitches bool) (playerWon bool) {
	winningDoor := pickRandomIndex()
	playerChoice := pickRandomIndex()
	//removedDoor := pickDoorIndexToRemove(winningDoor, playerChoice)
	if playerSwitches {
		return playerChoice != winningDoor
	}
	return playerChoice == winningDoor
}
