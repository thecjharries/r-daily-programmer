package main

import (
	"fmt"
	"io/ioutil"
	"path"
	"regexp"
)

var pathToText string = path.Join(".", "project-gutenberg_the-adventures-of-sherlock-holmes.txt")
var headerPattern *regexp.Regexp = regexp.MustCompile(`(?ms).+?^\*\*\* START.+?$`)
var footerPattern *regexp.Regexp = regexp.MustCompile(`(?ms)^\*\*\* END.+$.+`)
var allowedCharactersPattern = regexp.MustCompile(`(?ims)[a-z0-9]`)

func main() {
	haystack := loadFileIntoString(pathToText)
	haystack = stripPatternFromString(headerPattern, haystack)
	haystack = stripPatternFromString(footerPattern, haystack)
	fmt.Println(haystack)
	fmt.Printf("Characters in Sherlock Holmes: %d", countAllowedCharacters(allowedCharactersPattern, haystack))
}

func loadFileIntoString(pathToFile string) string {
	content, _ := ioutil.ReadFile(pathToFile)
	return string(content)
}

func stripPatternFromString(patternToRemove *regexp.Regexp, haystack string) string {
	patternToRemove.ReplaceAllString(haystack, "")
	return haystack
}

func countAllowedCharacters(patternForAllowedCharacters *regexp.Regexp, haystack string) int {
	return len(patternForAllowedCharacters.FindAllStringIndex(haystack, -1))
}
