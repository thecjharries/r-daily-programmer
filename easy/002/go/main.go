package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("hello world")
}

func getStringInput(prompt string, source io.Reader) string {
	reader := bufio.NewReader(source)
	fmt.Println(prompt)
	input, _ := reader.ReadString('\n')
	return strings.Replace(input, "\n", "", -1)
}

func getFloat64Input(prompt string, source io.Reader) float64 {
	stringResult := getStringInput(prompt, source)
	float64result, _ := strconv.ParseFloat(stringResult, 64)
	return float64result
}

func roundToUsd(input float64) float64 {
	return math.Round(input * 100) / 100
}

func getUsdPercentage(base float64, percent float64) float64 {
	usdBase := roundToUsd(base)
	return roundToUsd(usdBase * percent / 100)
}
