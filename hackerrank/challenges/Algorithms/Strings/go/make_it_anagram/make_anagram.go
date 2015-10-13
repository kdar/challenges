package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

var (
	TEST = `cde
abc`
)

func solve(input io.Reader) {
	scanner := bufio.NewScanner(input)
	scanner.Scan()
	string1 := scanner.Text()

	scanner.Scan()
	string2 := scanner.Text()

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
	for c, _ := range counts1 {
		current := counts1[c] - counts2[c]
		if current > 0 {
			deleted += current
		}
	}
	fmt.Printf("%d\n", deleted)
}

func main() {
	if len(os.Args) > 1 && os.Args[1] == "test" {
		solve(strings.NewReader(TEST))
		return
	}

	solve(os.Stdin)
}
