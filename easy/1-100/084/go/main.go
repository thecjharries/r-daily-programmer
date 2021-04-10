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
	"os"

	"github.com/thecjharries/dprgstd/clinput"
)

type Action func() Action

var northSouth int = 0
var eastWest int = 0

var actions = map[string]Action{
	"north": moveNorth,
	"south": moveSouth,
	"east":  moveEast,
	"west":  moveWest,
	"quit":  quit,
}

var zPrint = fmt.Println

func main() {
	currentAction := getAction
	for true {
		newAction := currentAction()
		if nil == newAction {
			newAction = getAction
		}
		currentAction = newAction
	}
}

func getAction() Action {
	actionName := clinput.GetStringInput("Which direction will you go?", os.Stdin)
	action, actionExists := actions[actionName]
	if actionExists {
		return action
	}
	return getAction
}

func printLocation() Action {
	_, _ = zPrint(fmt.Sprintf("You are currently at %d, %d", northSouth, eastWest))
	return nil
}

func moveDirection(ns, ew int) Action {
	northSouth += ns
	eastWest += ew
	return printLocation
}

func moveNorth() Action {
	return moveDirection(1, 0)
}

func moveSouth() Action {
	return moveDirection(-1, 0)
}

func moveEast() Action {
	return moveDirection(0, 1)
}

func moveWest() Action {
	return moveDirection(0, -1)
}

func quit() Action {
	os.Exit(0)
	return nil
}
