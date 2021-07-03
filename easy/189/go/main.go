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

type HangmanGame struct {
	ChosenWord       string
	GuessedLetters   map[rune]bool
	GuessesRemaining int
}

func (g *HangmanGame) String() string {
	representation := g.ChosenWord
	for _, character := range representation {
		_, guessed := g.GuessedLetters[character]
		if !guessed {
			representation = strings.ReplaceAll(representation, string(character), "_")
		}
	}
	return fmt.Sprintf("%s\nGuesses remaining: %d", representation, g.GuessesRemaining)
}

func (g *HangmanGame) Guess(letter rune) string {
	_, guessed := g.GuessedLetters[letter]
	if 0 < g.GuessesRemaining && !guessed {
		g.GuessesRemaining--
		inWord := false
		for _, character := range g.ChosenWord {
			if letter == character {
				inWord = true
				break
			}
		}
		g.GuessedLetters[letter] = inWord
	}
	return g.String()
}

func NewGame(wordToPlay string, availableGuesses int) *HangmanGame {
	return &HangmanGame{
		ChosenWord:       wordToPlay,
		GuessedLetters:   nil,
		GuessesRemaining: availableGuesses,
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
