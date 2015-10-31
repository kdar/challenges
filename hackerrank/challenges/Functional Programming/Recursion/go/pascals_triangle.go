package main

import "fmt"

// import (
// 	"fmt"
// 	"strings"
// )

// func printRow(row []int) {
// 	rowstr := []string{}
// 	for _, i := range row {
// 		rowstr = append(rowstr, fmt.Sprintf("%d", i))
// 	}

// 	fmt.Println(strings.Join(rowstr, " "))
// }

// func sum(a, b []int) []int {
// 	c := []int{}
// 	for i, _ := range a {
// 		c = append(c, a[i]+b[i])
// 	}
// 	return c
// }

// func main() {
// 	step := []int{1}
// 	for i := 0; i < 5; i++ {
// 		printRow(step)
// 		step = sum(append(step, 0), append([]int{0}, step...))
// 	}
// }

func triangulo(n, k int) int {
	if k == 0 || k == n {
		return 1
	}
	return triangulo(n-1, k-1) + triangulo(n-1, k)
}

func main() {
	num := 0
	fmt.Scanf("%d", &num)
	for n := 0; n < num; n++ {
		for k := 0; k < n+1; k++ {
			fmt.Printf("%d ", triangulo(n, k))
		}
		fmt.Println("")
	}
}
