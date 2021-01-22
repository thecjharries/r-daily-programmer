package main

import "fmt"

func main() {
	fmt.Println("hello world")
}

// Mostly from
// https://www.golangprograms.com/golang-program-to-print-all-permutations-of-a-given-string.html
func insertCharacterIntoEveryPositionOfString(existingString []rune, newCharacter rune) (permutations []string) {
	for index := 0; index <= len(existingString); index++ {
		permutations = append(
			permutations,
			string(existingString[:index]) + string(newCharacter) + string(existingString[index:]),
		)
	}
	return
}

// Mostly from
// https://www.golangprograms.com/golang-program-to-print-all-permutations-of-a-given-string.html
func permuteString(remaining []rune, permutations []string) []string {
	if 0 == len(remaining) {
		return permutations
	}
	var newPermutations []string
	for _, permutation := range permutations {
		newPermutations = append(
			newPermutations,
			insertCharacterIntoEveryPositionOfString([]rune(permutation), remaining[0])...,
		)
	}
	return permuteString(remaining[1:], newPermutations)
}
