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
	"regexp"
	"strings"
)

var sentenceStopPattern = regexp.MustCompile(`[.?!]`)

var promptInput = "If it will feed nothing else, it will\nfeed my revenge. He hath disgrac'd me and hind'red me half a\nmillion; laugh'd at my losses, mock'd at my gains, scorned my\nnation, thwarted my bargains, cooled my friends, heated mine\nenemies. And what's his reason? I am a Jew. Hath not a Jew eyes?\nHath not a Jew hands, organs, dimensions, senses, affections,\npassions, fed with the same food, hurt with the same weapons,\nsubject to the same diseases, healed by the same means, warmed\nand cooled by the same winter and summer, as a Christian is? If\nyou prick us, do we not bleed? If you tickle us, do we not laugh?\nIf you poison us, do we not die? And if you wrong us, shall we\nnot revenge? If we are like you in the rest, we will resemble you\nin that. If a Jew wrong a Christian, what is his humility?\nRevenge. If a Christian wrong a Jew, what should his sufferance\nbe by Christian example? Why, revenge. The villainy you teach me\nI will execute; and it shall go hard but I will better the\ninstruction."

func main() {
	longestSentence, characterLength := completePrompt(promptInput)
	fmt.Printf("The longet sentence, with %d characters, is as follows: %s", characterLength, longestSentence)
}

func sanitizeNewLines(input string) string {
	return strings.ReplaceAll(input, "\n", " ")
}

func explodeStringOnSentenceStopPattern(input string) []string {
	return sentenceStopPattern.Split(input, -1)
}

func findLargestSentence(input []string) string {
	longestSentence := ""
	for _, newSentence := range input {
		if len(longestSentence) < len(newSentence) {
			longestSentence = newSentence
		}
	}
	return longestSentence
}

func completePrompt(input string) (string, int) {
	exploded := explodeStringOnSentenceStopPattern(sanitizeNewLines(input))
	longestSentence := findLargestSentence(exploded)
	return longestSentence, len(longestSentence)
}
