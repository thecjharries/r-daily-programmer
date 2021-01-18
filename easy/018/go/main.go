package main

import (
	"fmt"
	"strings"
)

var phoneNumberToPureNumber = map[string]string{
	"a": "2",
	"b": "2",
	"c": "2",
	"d": "3",
	"e": "3",
	"f": "3",
	"g": "4",
	"h": "4",
	"i": "4",
	"j": "5",
	"k": "5",
	"l": "5",
	"m": "6",
	"n": "6",
	"o": "6",
	"p": "7",
	"q": "7",
	"r": "7",
	"s": "7",
	"t": "8",
	"u": "8",
	"v": "8",
	"w": "9",
	"x": "9",
	"y": "9",
	"z": "9",
	"1": "1",
	"2": "2",
	"3": "3",
	"4": "4",
	"5": "5",
	"6": "6",
	"7": "7",
	"8": "8",
	"9": "9",
	"0": "0",
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

func convertPhoneNumberToPureNumber(phoneNumber string) string {
	result := ""
	for _, character := range strings.ToLower(phoneNumber) {
		converted, exists := phoneNumberToPureNumber[string(character)]
		if exists {
			result += converted
		}
	}
	return result
}
