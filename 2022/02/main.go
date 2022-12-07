package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

type Move uint8
type Result uint8
type Fixed uint8

const (
	Rock     Move = 1
	Paper    Move = 2
	Scissors Move = 3
)

const (
	Win  Result = 6
	Lose Result = 0
	Draw Result = 3
)

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	totalScorePt1 := 0
	totalScorePt2 := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		playerMove := getVal(line[2:3])
		opponentMove := getVal(line[0:1])
		winner := findWinner(playerMove, opponentMove)

		totalScorePt1 += int(winner) + int(playerMove)

		winnerFixed := findWinnerFixed(playerMove, opponentMove)
		playerMoveFixed := getValFixed(playerMove, opponentMove)

		totalScorePt2 += int(winnerFixed) + int(playerMoveFixed)
	}

	fmt.Printf("Part 1: %d\n", totalScorePt1)
	fmt.Printf("Part 2: %d", totalScorePt2)
}

func getVal(move string) Move {
	switch move {
	case "A", "X":
		return Rock
	case "B", "Y":
		return Paper
	default:
		return Scissors
	}
}

func findWinner(player Move, opponent Move) Result {
	switch math.Abs(float64(player) - float64(opponent)) {
	case 0:
		return Draw
	case 1:
		if player > opponent {
			return Win
		}

		return Lose
	default:
		if player > opponent {
			return Lose
		}

		return Win
	}
}

func getValFixed(player Move, opponent Move) Move {
	switch player {
	case Rock:
		if opponent == Rock {
			return Move(opponent + 2)
		}

		return Move(opponent - 1)
	case Paper:
		return opponent
	default:
		if opponent == Scissors {
			return Move(opponent - 2)
		}

		return Move(opponent + 1)
	}
}

func findWinnerFixed(player Move, opponent Move) Result {
	switch player {
	case Rock:
		return Lose
	case Paper:
		return Draw
	default:
		return Win
	}
}
