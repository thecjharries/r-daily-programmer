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

var farmOccupants = map[string]string {
	"cow": "moo",
	"chicken": "cluck",
	"turkey": "gobble",
	"kangaroo": "g'day mate",
	"T-Rex": "GAAAAARGH",
	"dog": "bark",
}

// https://www.kidsongs.com/lyrics/old-macdonald-had-a-farm.html
var stanza = `Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a %[1]s
E-I-E-I-O
With a %[2]s %[2]s here
And a %[2]s %[2]s there
Here a %[2]s, there a %[2]s
Everywhere a %[2]s %[2]s
Old MacDonald had a farm
E-I-E-I-O

`

func main() {
	for animal, sound := range farmOccupants {
		fmt.Printf(stanza, animal, sound)
	}
}
