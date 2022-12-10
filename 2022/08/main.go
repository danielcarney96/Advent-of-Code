package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var grid [][]int

	for scanner.Scan() {
		line := scanner.Text()

		var chars []int

		for _, char := range line {
			chars = append(chars, int(char-'0'))
		}

		grid = append(grid, chars)
	}

	length := len(grid)
	visibleTrees := 0

	for x := 0; x < length; x++ {
		for y := 0; y < length; y++ {
			// on the boundary
			if x == 0 || y == 0 || x == length-1 || y == length-1 {
				visibleTrees++
				continue
			}

			// check top
			visible := true
			for z := 0; z < y; z++ {
				if grid[x][z] >= grid[x][y] {
					visible = false
				}
			}

			if visible {
				visibleTrees++
				continue
			}

			// check bottom
			visible = true
			for z := length - 1; z > y; z-- {
				if grid[x][z] >= grid[x][y] {
					visible = false
				}
			}

			if visible {
				visibleTrees++
				continue
			}

			// check left
			visible = true
			for z := 0; z < x; z++ {
				if grid[z][y] >= grid[x][y] {
					visible = false
				}
			}

			if visible {
				visibleTrees++
				continue
			}

			// check right
			visible = true
			for z := length - 1; z > x; z-- {
				if grid[z][y] >= grid[x][y] {
					visible = false
				}
			}

			if visible {
				visibleTrees++
				continue
			}
		}
	}

	fmt.Printf("Part 1: %d\n", visibleTrees)
}
