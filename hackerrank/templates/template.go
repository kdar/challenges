package main

import (
	"io"
	"os"
	"strings"
)

var (
	TEST = ``
)

func solve(input io.Reader) {
	// scanner := bufio.NewScanner(input)
	// scanner.Scan()
	// A := scanner.Text()
}

func main() {
	if len(os.Args) > 1 && os.Args[1] == "test" {
		solve(strings.NewReader(TEST))
		return
	}

	solve(os.Stdin)
}
