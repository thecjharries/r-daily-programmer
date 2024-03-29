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

type RpsGame map[string]map[string]string

// This provides an arbitrary value because maps are not sorted
// This is impossible to test reliably
func (r *RpsGame) ComputerMove() (move string) {
	for key := range *r {
		move = key
		break
	}
	return
}

func (r *RpsGame) Round(humanMove string) string {
	computerMove := r.ComputerMove()
	subject := humanMove
	action := "???"
	object := computerMove
	winner := "Neither"
	humanResult, humanOk := (*r)[humanMove][computerMove]
	if humanOk {
		action = humanResult
		winner = "human"
	}
	computerResult, computerOk := (*r)[computerMove][humanMove]
	if computerOk {
		subject = computerMove
		action = computerResult
		object = humanMove
		winner = "computer"
	}
	return fmt.Sprintf("%s %s %s; %s wins", subject, action, object, winner)
}

var RockPaperScissorsLizardSpock = RpsGame{
	"rock": {
		"scissors": "crushes",
		"lizard":   "crushes",
	},
	"paper": {
		"rock":  "covers",
		"spock": "disproves",
	},
	"scissors": {
		"paper":  "cuts",
		"lizard": "decapitates",
	},
	"lizard": {
		"paper": "eats",
		"spock": "poisons",
	},
	"spock": {
		"rock":     "vaporizes",
		"scissors": "smashes",
	},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint(RockPaperScissorsLizardSpock.Round("rock"))
	_, _ = zPrint(RockPaperScissorsLizardSpock.Round("paper"))
	_, _ = zPrint(RockPaperScissorsLizardSpock.Round("scissors"))
	_, _ = zPrint(RockPaperScissorsLizardSpock.Round("lizard"))
	_, _ = zPrint(RockPaperScissorsLizardSpock.Round("spock"))
}
