package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type position struct {
	y int
	x int
}

func main() {
	part1()
	part2()
}

func part1() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	startPosition := position{
		x: 0,
		y: 0,
	}

	var tailPositionsVisited []position
	tailPositionsVisited = append(tailPositionsVisited, startPosition)

	headPosition := startPosition
	tailPosition := startPosition

	for scanner.Scan() {
		line := scanner.Text()

		instruction := strings.Split(line, " ")
		direction := instruction[0]
		amount, _ := strconv.Atoi(instruction[1])

		for i := 0; i < amount; i++ {
			xMove := 0
			yMove := 0

			if direction == "L" {
				xMove = -1
			} else if direction == "R" {
				xMove = 1
			}

			if direction == "D" {
				yMove = 1
			} else if direction == "U" {
				yMove = -1
			}

			headPosition.x += xMove
			headPosition.y += yMove

			xMinus := headPosition.x - tailPosition.x
			yMinus := headPosition.y - tailPosition.y

			if math.Abs(float64(xMinus)) > 1 || math.Abs(float64(yMinus)) > 1 {
				if xMinus == 0 {
					tailPosition.y += int(math.Floor(float64(yMinus) / float64(2)))
				} else if yMinus == 0 {
					tailPosition.x += int(math.Floor(float64(xMinus) / float64(2)))
				} else {
					if xMinus > 0 {
						tailPosition.x += 1
					} else {
						tailPosition.x -= 1
					}

					if yMinus > 0 {
						tailPosition.y += 1
					} else {
						tailPosition.y -= 1
					}
				}
			}

			tailPositionsVisited = append(
				tailPositionsVisited,
				tailPosition,
			)
		}

	}

	var tailPositionsVisitedNoDuplicates []position

	for _, tailPositionVisited := range tailPositionsVisited {
		found := false

		for _, tailPositionVisitedNoDuplicates := range tailPositionsVisitedNoDuplicates {
			if tailPositionVisited == tailPositionVisitedNoDuplicates {
				found = true
				break
			}
		}

		if !found {
			tailPositionsVisitedNoDuplicates = append(tailPositionsVisitedNoDuplicates, tailPositionVisited)
		}
	}

	fmt.Printf("Part 1: %d\n", len(tailPositionsVisitedNoDuplicates))
}

func part2() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	startPosition := position{
		x: 0,
		y: 0,
	}

	var knotPositions []position

	for i := 0; i < 10; i++ {
		knotPositions = append(knotPositions, startPosition)
	}

	var tailPositionsVisited []position
	tailPositionsVisited = append(tailPositionsVisited, startPosition)

	for scanner.Scan() {
		line := scanner.Text()

		instruction := strings.Split(line, " ")
		direction := instruction[0]
		amount, _ := strconv.Atoi(instruction[1])

		for i := 0; i < amount; i++ {
			xMove := 0
			yMove := 0

			if direction == "L" {
				xMove = -1
			} else if direction == "R" {
				xMove = 1
			}

			if direction == "D" {
				yMove = 1
			} else if direction == "U" {
				yMove = -1
			}

			knotPositions[0].x += xMove
			knotPositions[0].y += yMove

			for k := 0; k < 9; k++ {
				xMinus := knotPositions[k].x - knotPositions[k+1].x
				yMinus := knotPositions[k].y - knotPositions[k+1].y

				if math.Abs(float64(xMinus)) > 1 || math.Abs(float64(yMinus)) > 1 {
					if xMinus == 0 {
						knotPositions[k+1].y += int(math.Floor(float64(yMinus) / float64(2)))
					} else if yMinus == 0 {
						knotPositions[k+1].x += int(math.Floor(float64(xMinus) / float64(2)))
					} else {
						if xMinus > 0 {
							knotPositions[k+1].x += 1
						} else {
							knotPositions[k+1].x -= 1
						}

						if yMinus > 0 {
							knotPositions[k+1].y += 1
						} else {
							knotPositions[k+1].y -= 1
						}
					}
				}

				if k+1 == len(knotPositions)-1 {
					tailPositionsVisited = append(
						tailPositionsVisited,
						knotPositions[k+1],
					)
				}
			}
		}

	}

	var tailPositionsVisitedNoDuplicates []position

	for _, tailPositionVisited := range tailPositionsVisited {
		found := false

		for _, tailPositionVisitedNoDuplicates := range tailPositionsVisitedNoDuplicates {
			if tailPositionVisited == tailPositionVisitedNoDuplicates {
				found = true
				break
			}
		}

		if !found {
			tailPositionsVisitedNoDuplicates = append(tailPositionsVisitedNoDuplicates, tailPositionVisited)
		}
	}

	fmt.Printf("Part 2: %d\n", len(tailPositionsVisitedNoDuplicates))
}
