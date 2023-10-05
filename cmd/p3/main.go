package main

import (
	"fmt"
	"strings"
)

type Icon = int

const (
	Tree Icon = iota
	Snow
)

func getInput() string {
	return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`
}

func main() {
	treeCount := 0

	for rowIndex, line := range strings.Split(getInput(), "\n") {
		if string(line[rowIndex*3%len(line)]) == "#" {
			treeCount += 1
		}
	}

	fmt.Printf("treecount %v\n", treeCount)
}
