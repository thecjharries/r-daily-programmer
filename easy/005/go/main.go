package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"path"
	"strings"
)

type Credentials struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

var validCredsPath = path.Join(".", "valid-creds.json")

func main() {
	fmt.Println("THIS IS NOT A SECURE PROGRAM")
	fmt.Println("ALSO THIS IS A BAD AUTH PATTERN")
	var creds Credentials
	creds.Username = getStringInput("Username?", os.Stdin)
	// better way without exposing password
	// https://stackoverflow.com/a/32768479
	creds.Password = getStringInput("Password?", os.Stdin)
	validCreds := loadValidCredentials(validCredsPath)
	if areTheseCredentialsValid(creds, validCreds) {
		fmt.Println("rad")
	} else {
		fmt.Println("no soup for you")
	}
}

func getStringInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func loadValidCredentials(credsPath string) []Credentials {
	handle, _ := os.Open(credsPath)
	defer (func(){ _ = handle.Close() })()
	byteArray, _ := ioutil.ReadAll(handle)
	var credentials []Credentials
	_ = json.Unmarshal(byteArray, &credentials)
	return credentials
}

func areTheseCredentialsValid(creds Credentials, validCredentials []Credentials) bool {
	for _, validCreds := range validCredentials {
		if validCreds.Username == creds.Username && validCreds.Password == creds.Password {
			return true
		}
	}
	return false
}
