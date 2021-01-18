package main

import "fmt"

var letterToNumber = map[string]int{
	"a": 2,
	"b": 2,
	"c": 2,
	"d": 3,
	"e": 3,
	"f": 3,
	"g": 4,
	"h": 4,
	"i": 4,
	"j": 5,
	"k": 5,
	"l": 5,
	"m": 6,
	"n": 6,
	"o": 6,
	"p": 7,
	"q": 7,
	"r": 7,
	"s": 7,
	"t": 8,
	"u": 8,
	"v": 8,
	"w": 9,
	"x": 9,
	"y": 9,
	"z": 9,
}

func main() {
	fmt.Println("hello world")
}

func formatPhoneNumber(phoneNumber string) string {
	countryCode := phoneNumber[:len(phoneNumber) - 10]
	areaCode := phoneNumber[len(phoneNumber) - 10:len(phoneNumber) - 7]
	almostLastThree := phoneNumber[len(phoneNumber) - 7:len(phoneNumber) - 4]
	lastFour := phoneNumber[len(phoneNumber) - 4:]
	return fmt.Sprintf("%s-%s-%s-%s", countryCode, areaCode, almostLastThree, lastFour)
}
