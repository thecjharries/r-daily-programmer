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
	phoneNumber := "1-800-COMCAST"
	converted := convertPhoneNumberToPureNumber(phoneNumber)
	pureNumber := formatPhoneNumber(converted)
	fmt.Printf("Original number:  %s\nConverted number: %s\n", phoneNumber, pureNumber)
}

func formatPhoneNumber(phoneNumber string) string {
	countryCode := phoneNumber[:len(phoneNumber)-10]
	areaCode := phoneNumber[len(phoneNumber)-10 : len(phoneNumber)-7]
	almostLastThree := phoneNumber[len(phoneNumber)-7 : len(phoneNumber)-4]
	lastFour := phoneNumber[len(phoneNumber)-4:]
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
