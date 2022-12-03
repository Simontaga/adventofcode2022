package days

import (
	"io/ioutil"
	"log"
	"strings"
)

const ENEMY_ROCK string = "A"
const ENEMY_PAPER string = "B"
const ENEMY_SCISSORS string = "C"

const PLAYER_ROCK string = "X"
const PLAYER_PAPER string = "Y"
const PLAYER_SCISSORS string = "Z"

const WIN_POINT int = 6
const DRAW_POINT int = 3
const LOSE_POINT int = 0

const ROCK_MATCH_POINTS int = 1
const PAPER_MATCH_POINTS int = 2
const SCISSOR_MATCH_POINTS int = 3

func Day_two_part_1() int {

	points := 0

	for _, value := range getInput_() {
		match := strings.Split(value, " ")
		enemy_move := match[0]
		player_move := match[1]
		// I hate this.

		if player_move == PLAYER_PAPER {
			switch enemy_move {
			case ENEMY_PAPER: // We draw
				points += DRAW_POINT
				points += PAPER_MATCH_POINTS
			case ENEMY_ROCK: // We win
				points += WIN_POINT
				points += PAPER_MATCH_POINTS
			case ENEMY_SCISSORS: // We Lose
				points += LOSE_POINT
				points += PAPER_MATCH_POINTS
			}
			continue
		}

		if player_move == PLAYER_ROCK {
			switch enemy_move {
			case ENEMY_PAPER: // We lose
				points += LOSE_POINT
				points += ROCK_MATCH_POINTS
			case ENEMY_ROCK: // We draw
				points += DRAW_POINT
				points += ROCK_MATCH_POINTS
			case ENEMY_SCISSORS: // We win
				points += WIN_POINT
				points += ROCK_MATCH_POINTS
			}
			continue
		}

		if player_move == PLAYER_SCISSORS {
			switch enemy_move {
			case ENEMY_PAPER: // We win
				points += WIN_POINT
				points += SCISSOR_MATCH_POINTS
			case ENEMY_ROCK: // We lose
				points += LOSE_POINT
				points += SCISSOR_MATCH_POINTS
			case ENEMY_SCISSORS: // We draw
				points += DRAW_POINT
				points += SCISSOR_MATCH_POINTS
			}
			continue
		}
	}

	return points
}

func Day_two_part_2() int {

	points := 0

	for _, value := range getInput_() {
		match := strings.Split(value, " ")
		enemy_move := match[0]
		player_move := match[1]

		// I hate this too.
		if player_move == PLAYER_ROCK {
			// Lose
			points += LOSE_POINT

			if enemy_move == ENEMY_PAPER {
				points += ROCK_MATCH_POINTS
			}

			if enemy_move == ENEMY_SCISSORS {
				points += PAPER_MATCH_POINTS
			}

			if enemy_move == ENEMY_ROCK {
				points += SCISSOR_MATCH_POINTS
			}
		}

		if player_move == PLAYER_PAPER {
			// DRAW
			points += DRAW_POINT

			if enemy_move == ENEMY_ROCK {
				points += ROCK_MATCH_POINTS
			}

			if enemy_move == ENEMY_PAPER {
				points += PAPER_MATCH_POINTS
			}

			if enemy_move == ENEMY_SCISSORS {
				points += SCISSOR_MATCH_POINTS
			}
		}

		if player_move == PLAYER_SCISSORS {
			// WIN
			points += WIN_POINT

			if enemy_move == ENEMY_ROCK {
				points += PAPER_MATCH_POINTS
			}

			if enemy_move == ENEMY_PAPER {
				points += SCISSOR_MATCH_POINTS
			}

			if enemy_move == ENEMY_SCISSORS {
				points += ROCK_MATCH_POINTS
			}
		}
	}

	return points
}

func getInput_() []string {
	content, err := ioutil.ReadFile("days/input/2.txt")

	if err != nil {
		log.Fatal(err)
	}

	var split = strings.Split(string(content), "\n")
	return split
}
