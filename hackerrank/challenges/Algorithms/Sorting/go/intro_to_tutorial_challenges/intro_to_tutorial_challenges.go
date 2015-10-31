package main

import "fmt"

func main() {
	var V, n int
	fmt.Scan(&V)
	fmt.Scan(&n)
	for i := 0; i < n; i++ {
		var d int
		fmt.Scan(&d)
		if d == V {
			fmt.Println(i)
			return
		}
	}
}
