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
	"strconv"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func buildPlacesNotWon(place int) string {
	var placesNotWon []string
	for index := 0; index < 101; index++ {
		if place == index {
			continue
		}
		placeNotWon := strconv.Itoa(index)
		if strings.HasSuffix(placeNotWon, "1") {
			placeNotWon += "st"
		} else if strings.HasSuffix(placeNotWon, "3") {
			placeNotWon += "rd"
		} else if strings.HasSuffix(placeNotWon, "2") {
			placeNotWon += "nd"
		} else {
			placeNotWon += "th"
		}
		placesNotWon = append(placesNotWon, placeNotWon)
	}
	return strings.Join(placesNotWon, ", ")
}
