package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	total := getFloat64Input("Meal total?", io.Reader(os.Stdin))
	desiredPercent := getFloat64Input("Desired tip percent?", io.Reader(os.Stdin))
	tipPercent15 := getUsdPercentage(total, 15)
	tipPercent20 := getUsdPercentage(total, 20)
	tipPercent25 := getUsdPercentage(total, 25)
	tipPercentDesired := getUsdPercentage(total, desiredPercent)
	fmt.Printf("15.00%% tip: %.2f\n", tipPercent15)
	fmt.Printf("20.00%% tip: %.2f\n", tipPercent20)
	fmt.Printf("25.00%% tip: %.2f\n", tipPercent25)
	fmt.Printf("%.2f%% tip: %.2f\n", desiredPercent, tipPercentDesired)
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
