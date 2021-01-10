package main

import (
	"fmt"
	"regexp"
)

// These patterns come directly from the prompt
// They're much too prescriptive and I don't like them.
var allowedFormPatterns = []string{
	`\d{9}`,
	`\d{3}-\d{3}-\d{4}`,
	`\d{3}\.\d{3}\.\d{4}`,
	`\(\d{3}\)\s{0,1}\d{3}-\d{4}`,
	`\d{3}\-\d{4}`,
}

func main() {
	fmt.Println("hello world")
}

func compilePatterns(patterns []string) []*regexp.Regexp {
	var allowedFormRegexp []*regexp.Regexp
	for _, pattern := range allowedFormPatterns {
		newRegexp, _ := regexp.CompilePOSIX(pattern)
		allowedFormRegexp = append(allowedFormRegexp, newRegexp)
	}
	return allowedFormRegexp
}

func validateNumber(phoneNumber string, allowedFormRegexp []*regexp.Regexp) bool {
	for _, formRegexp := range allowedFormRegexp {
		if formRegexp.MatchString(phoneNumber) {
			return true
		}
	}
	return false
}
