package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

var (
	TEST = `6
aaabbb
ab
abc
mnop
xyyx
xaxbbbxx`
)

func solve(input io.Reader) {
	scanner := bufio.NewScanner(input)
	scanner.Scan()
	line := scanner.Text()
	testCases, _ := strconv.Atoi(line)

	for i := 0; i < testCases; i++ {
		scanner.Scan()
		line := scanner.Text()

		if len(line)%2 != 0 {
			fmt.Println("-1")
		} else {
			string1 := line[0 : len(line)/2]
			string2 := line[len(line)/2:]

			counts1 := make(map[rune]int)
			counts2 := make(map[rune]int)

			for _, c := range string1 {
				counts1[c] += 1
			}
			for _, c := range string2 {
				counts2[c] += 1
			}

			deleted := 0
			for c, _ := range counts2 {
				current := counts2[c] - counts1[c]
				if current > 0 {
					deleted += current
				}
			}
			fmt.Printf("%d\n", deleted)
		}
	}
}

func main() {
	if len(os.Args) > 1 && os.Args[1] == "test" {
		solve(strings.NewReader(TEST))
		return
	}

	solve(os.Stdin)
}
