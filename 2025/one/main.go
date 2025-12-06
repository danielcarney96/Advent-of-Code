package main

import (
	"2025/reader"
	"fmt"
	"strconv"
)

type Direction int

const (
	Left Direction = iota
	Right
)

type Instruction struct {
	direction Direction
	points    int
}

const START_POSITION = 50

func main() {
	data, err := reader.ReadFromFile("./input.txt")

	if err != nil {
		fmt.Print(err)
		return
	}

	currentPosition := START_POSITION
	zeroHits1 := 0
	zeroHits2 := 0

	for _, row := range data {
		firstChar := row[0]
		var dir Direction

		if firstChar == 'L' {
			dir = Left
		} else {
			dir = Right
		}

		pointsInt, err := strconv.Atoi(row[1:])

		if err != nil {
			fmt.Print(err)
			return
		}

		instruction := Instruction{
			direction: dir,
			points:    pointsInt,
		}

		updatedPosition, newHits1, newHits2 := turnDial(instruction, currentPosition)

		currentPosition = updatedPosition
		zeroHits1 = zeroHits1 + newHits1
		zeroHits2 = zeroHits2 + newHits2
	}

	fmt.Println(zeroHits1)
	fmt.Println(zeroHits2)
}

func turnDial(instruction Instruction, currentPosition int) (int, int, int) {
	const DIAL_MAX = 99
	const DIAL_MIN = 0

	zeroHitsPart1 := 0
	zeroHitsPart2 := 0

	for i := 0; i < instruction.points; i++ {
		if instruction.direction == Left {
			currentPosition = currentPosition - 1

			if currentPosition < DIAL_MIN {
				currentPosition = DIAL_MAX
			}
		}

		if instruction.direction == Right {
			currentPosition = currentPosition + 1

			if currentPosition > DIAL_MAX {
				currentPosition = DIAL_MIN
			}
		}

		if currentPosition == DIAL_MIN {
			zeroHitsPart2 = zeroHitsPart2 + 1
		}
	}

	if currentPosition == DIAL_MIN {
		zeroHitsPart1 = zeroHitsPart1 + 1
	}

	return currentPosition, zeroHitsPart1, zeroHitsPart2
}
