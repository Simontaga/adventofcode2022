package days

import (
	"io/ioutil"
	"log"
	"sort"
	"strconv"
	"strings"
)

func Day_one_part_1() int {

	highest := 0
	current := 0

	for _, value := range getInput() {
		if value == "" {
			if current > highest {
				highest = current
			}
			current = 0
			continue
		}

		intVar, err := strconv.Atoi(value)
		if err != nil {
			log.Fatal(err)
		}

		current += intVar
	}

	return highest
}

func Day_one_part_2() int {

	current := 0
	elves := []int{}

	for _, value := range getInput() {
		if value == "" {
			elves = append(elves, current)
			current = 0
			continue
		}

		intVar, err := strconv.Atoi(value)
		if err != nil {
			log.Fatal(err)
		}

		current += intVar
	}

	sort.Ints(elves)

	// Very nice.
	return elves[len(elves)-3] + elves[len(elves)-2] + elves[len(elves)-1]
}

func getInput() []string {
	content, err := ioutil.ReadFile("days/input/1.txt")

	if err != nil {
		log.Fatal(err)
	}

	var split = strings.Split(string(content), "\n")
	return split
}
