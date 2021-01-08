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
	fmt.Println("hello world")
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
