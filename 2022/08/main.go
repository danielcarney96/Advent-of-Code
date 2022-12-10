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

	fmt.Printf("Part 1: %d\n", calculateVisibleTrees(grid))
	fmt.Printf("Part 2: %d\n", findHighestScore(grid))
}

func calculateVisibleTrees(grid [][]int) int {
	visibleTrees := 0
	length := len(grid)

	for y := 0; y < length; y++ {
		for x := 0; x < length; x++ {
			// on the boundary
			if x == 0 || y == 0 || x == length-1 || y == length-1 {
				visibleTrees++
				continue
			}

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

	return visibleTrees
}

func findHighestScore(grid [][]int) int {
	var score int
	length := len(grid)

	for y := 0; y < length; y++ {
		for x := 0; x < length; x++ {
			// any boundary will sum to 0 so skip it
			if x == 0 || x == length-1 || y == 0 || y == length-1 {
				continue
			}

			// check top
			visibleTop := 0
			for z := y - 1; z >= 0; z-- {
				visibleTop++
				if grid[z][x] >= grid[y][x] {
					break
				}
			}

			// check bottom
			visibleBottom := 0
			for z := y + 1; z < length; z++ {
				visibleBottom++
				if grid[z][x] >= grid[y][x] {
					break
				}
			}

			// check left
			visibleLeft := 0
			for z := x - 1; z >= 0; z-- {
				visibleLeft++
				if grid[y][z] >= grid[y][x] {
					break
				}
			}

			// check right
			visibleRight := 0
			for z := x + 1; z < length; z++ {
				visibleRight++
				if grid[y][z] >= grid[y][x] {
					break
				}
			}

			currentScore := visibleBottom * visibleTop * visibleLeft * visibleRight

			if currentScore > score {
				score = currentScore
			}
		}
	}

	return score
}
