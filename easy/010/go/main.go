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
