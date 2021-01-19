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
