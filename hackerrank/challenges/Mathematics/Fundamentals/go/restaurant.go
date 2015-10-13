package main

import (
	"fmt"
)

func main() {
	T := 0
	fmt.Scan(&T)

	for x := 0; x < T; x++ {
		l, b := 0, 0
		fmt.Scan(&l, &b)

		if l == b {
			fmt.Println("1")
			continue
		}

		answer := l * b
		area := l * b
		for i := 2; true; i++ {
			i2 := i * i
			if i2 > area {
				break
			}

			if area%i2 == 0 && l%i == 0 && b%i == 0 {
				answer = area / (i * i)
			}
		}

		fmt.Println(answer)
	}
}
