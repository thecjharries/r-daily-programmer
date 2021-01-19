package main

import (
	"fmt"
	"io/ioutil"
	"path"
)

var pathToText string = path.Join(".", "project-gutenberg_the-adventures-of-sherlock-holmes.txt")

func main() {
	fmt.Println("hello world")
}

func loadFileIntoString(pathToFile string) string {
	content, _ := ioutil.ReadFile(pathToFile)
	return string(content)
}
