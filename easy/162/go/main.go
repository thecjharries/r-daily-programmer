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
	"strconv"
	"strings"
)

const (
	PatternMatchWhole int = iota
	PatternMatchGroup
	PatternMatchWord
	PatternMatchCase
	PatternMatchNewline
	PatternMatchEof
	PatternMatchPunctuation
)

var compressionPattern = regexp.MustCompile(`((?P<word>\d+)(?P<case>[\^!]?)|(?P<newline>R)|(?P<eof>E)|(?P<punctuation>[^\d\s]))`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

// Matches return as the tuple
// (group, entire match, <word>, <case>, <newline>, <eof>, <punctuation>)
// Go obfuscates named groups behind a bunch of junk
func decompress(input string, dictionary []string) (output string) {
	matches := compressionPattern.FindAllStringSubmatch(input, -1)
	for _, match := range matches {
		// We don't care about the whole match, the group, or EOF
		if "" != match[PatternMatchWord] {
			dictionaryIndex, _ := strconv.Atoi(match[PatternMatchWord])
			word := dictionary[dictionaryIndex]
			if "^" == match[PatternMatchCase] {
				word = strings.Title(word)
			} else if "!" == match[PatternMatchCase] {
				word = strings.ToUpper(word)
			}
			output += fmt.Sprintf(" %s", word)
		} else if "" != match[PatternMatchNewline] {
			output += "\n"
		} else if "" != match[PatternMatchPunctuation] {
			output += match[PatternMatchPunctuation]
		}
	}
	return strings.TrimSpace(output)
}
