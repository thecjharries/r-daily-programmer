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
var allowedCharactersPattern = regexp.MustCompile(`(?im)[a-z0-9]`)

func main() {
	fmt.Println("hello world")
}

func loadFileIntoString(pathToFile string) string {
	content, _ := ioutil.ReadFile(pathToFile)
	return string(content)
}

func stripPatternFromString(patternToRemove *regexp.Regexp, haystack string) string {
	return patternToRemove.ReplaceAllString(haystack, "")
}

func countAllowedCharacters(patternForAllowedCharacters *regexp.Regexp, haystack string) int {
	return len(patternForAllowedCharacters.FindAllStringIndex(haystack, -1))
}
