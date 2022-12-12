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

		tailPositionsVisited = append(
			tailPositionsVisited,
			makeMove(&headPosition, &tailPosition, amount, direction)...,
		)
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

func makeMove(leadingPosition *position, trailingPosition *position, amount int, direction string) []position {
	var trailingMovesMade []position

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

		leadingPosition.x += xMove
		leadingPosition.y += yMove

		xMinus := leadingPosition.x - trailingPosition.x
		yMinus := leadingPosition.y - trailingPosition.y

		if math.Abs(float64(xMinus)) > 1 || math.Abs(float64(yMinus)) > 1 {
			if xMinus == 0 {
				trailingPosition.y += int(math.Floor(float64(yMinus) / float64(2)))
			} else if yMinus == 0 {
				trailingPosition.x += int(math.Floor(float64(xMinus) / float64(2)))
			} else {
				if xMinus > 0 {
					trailingPosition.x += 1
				} else {
					trailingPosition.x -= 1
				}

				if yMinus > 0 {
					trailingPosition.y += 1
				} else {
					trailingPosition.y -= 1
				}
			}
		}

		trailingMovesMade = append(
			trailingMovesMade,
			*trailingPosition,
		)
	}

	return trailingMovesMade
}
