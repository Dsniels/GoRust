package main

import (
	"fmt"
	"strings"
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

type Thing = int

const (
	Tree Thing = iota
	Snow
)

func main() {
	treeCount := 0
	for i, line := range strings.Split(getInput(), "\n") {
		if string(line[i*3%len(line)]) == "#" {
			treeCount += 1
		}

	}

	fmt.Printf("treecount %v\n", treeCount)
}
