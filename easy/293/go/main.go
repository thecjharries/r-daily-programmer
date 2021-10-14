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

var nextAllowedCables = map[string][]string{
	"white":  {"red", "orange", "green", "purple"},
	"red":    {"green"},
	"black":  {"red", "purple", "black"},
	"orange": {"red", "black"},
	"green":  {"orange", "white"},
	"purple": {"red", "black"},
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func wasBombDefused(wires []string) bool {
	currentAllowedWires, exists := nextAllowedCables[wires[0]]
	if !exists {
		return false
	}
	for _, wire := range wires[1:] {
		wireAllowed := false
		for _, possibleWire := range currentAllowedWires {
			if wire == possibleWire {
				wireAllowed = true
			}
		}
		if !wireAllowed {
			return false
		}
		currentAllowedWires, exists = nextAllowedCables[wire]
		if !exists {
			return false
		}
	}
	return true
}
