package cryptosquare

import (
	"math"
	"strings"
	"unicode"
)

const TestVersion = 1

func colcount(input string) int {
	return int(math.Ceil(math.Sqrt(float64(len(input)))))
}

func Encode(input string) string {
	input = strings.Map(func(r rune) rune {
		if unicode.IsLetter(r) || unicode.IsDigit(r) {
			return unicode.ToLower(r)
		}
		return -1
	}, input)

	cols := colcount(input)
	grid := make([]string, cols)
	for i, r := range input {
		grid[i%cols] += string(r)
	}
	return strings.Join(grid, " ")
}
