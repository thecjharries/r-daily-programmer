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

var chromaticScale = []string{"C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"}
var solfegeScale = []string{"Do", "Re", "Mi", "Fa", "So", "La", "Ti"}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func buildMajorScale(note string) (notes []string) {
	initialIndex := 0
	for index, scaleNote := range chromaticScale {
		if note == scaleNote {
			initialIndex = index
			break
		}
	}
	for _, noteOffset := range []int{0, 2, 4, 5, 7, 9, 11} {
		notes = append(notes, chromaticScale[(initialIndex+noteOffset)%len(chromaticScale)])
	}
	return
}

func note(scale, solfege string) (note string) {
	majorScale := buildMajorScale(scale)
	for index, solfegeNote := range solfegeScale {
		if solfege == solfegeNote {
			note = majorScale[index]
			break
		}
	}
	return
}
