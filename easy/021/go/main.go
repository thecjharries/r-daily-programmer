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
func permuteStringRecursion(remaining []rune, permutations []string) []string {
	fmt.Println(string(remaining))
	if 0 == len(remaining) {
		return permutations
	}
	var newPermutations []string
	for _, permutation := range permutations {
		fmt.Println(permutation)
		newPermutations = append(
			newPermutations,
			insertCharacterIntoEveryPositionOfString([]rune(permutation), remaining[0])...,
		)
	}
	return permuteStringRecursion(remaining[1:], newPermutations)
}

// Mostly from
// https://www.golangprograms.com/golang-program-to-print-all-permutations-of-a-given-string.html
func permuteString(stringToPermute string) []string {
	asRune := []rune(stringToPermute)
	return permuteStringRecursion(asRune[1:], []string{string(asRune[0])})
}
