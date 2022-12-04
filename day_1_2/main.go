package main

import (
	"adventofcode/days"
	"fmt"
	"time"
)

func main() {
	start := time.Now()
	fmt.Println(days.Day_one_part_1())
	fmt.Printf("Took:%s\n", time.Since(start))

	start = time.Now()
	fmt.Println(days.Day_one_part_2())
	fmt.Printf("Took:%s\n", time.Since(start))

	start = time.Now()
	fmt.Println(days.Day_two_part_1())
	fmt.Printf("Took:%s\n", time.Since(start))

	start = time.Now()
	fmt.Println(days.Day_two_part_2())
	fmt.Printf("Took:%s\n", time.Since(start))
}
