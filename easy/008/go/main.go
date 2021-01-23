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

// https://en.wikipedia.org/wiki/99_Bottles_of_Beer
const (
	nthLyric string = `%d bottles of beer on the wall, %d bottles of beer.
Take one down, pass it around, %d bottles of beer on the wall...
`
	lastLyric string = `No more bottles of beer on the wall, no more bottles of beer.
We've taken them down and passed them around; now we're drunk and passed out!
`
	errorBadBottleCount string = "Unable to use this bottleCount"
)

func main() {
	fmt.Print(generateSong(100))
}

func generateLyrics(bottleCount int) string {
	if 0 < bottleCount {
		return fmt.Sprintf(nthLyric, bottleCount, bottleCount, bottleCount - 1)
	}
	if 0 > bottleCount {
		panic(errorBadBottleCount)
	}
	return lastLyric
}

func generateSong(bottleCount int) string {
	song := ""
	for currentCount := bottleCount; currentCount > -1; currentCount-- {
		song += generateLyrics(currentCount)
	}
	return song
}
