import argparse
import enum
import pathlib


class Move(enum.IntEnum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3


class Result(enum.IntEnum):
    LOSS = 0
    DRAW = 3
    WIN = 6


def calculate_strategy_score(strategy_file):
    opponent_moves = {
        'A': Move.ROCK,
        'B': Move.PAPER,
        'C': Move.SCISSORS,
    }
    player_moves = {
        'X': Move.ROCK,
        'Y': Move.PAPER,
        'Z': Move.SCISSORS,
    }
    round_scores = {
        # (opponent, player): player_score
        (Move.ROCK, Move.SCISSORS): Result.LOSS,
        (Move.SCISSORS, Move.PAPER): Result.LOSS,
        (Move.PAPER, Move.ROCK): Result.LOSS,
        (Move.SCISSORS, Move.ROCK): Result.WIN,
        (Move.PAPER, Move.SCISSORS): Result.WIN,
        (Move.ROCK, Move.PAPER): Result.WIN,
    }
    score = 0
    for line in strategy_file:
        opponent, player = line.strip().split(maxsplit=1)
        opponent = opponent_moves[opponent]
        player = player_moves[player]
        score += player.value + round_scores.get((opponent, player), Result.DRAW).value
    return score


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path to strategy file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as strategy_file:
        score = calculate_strategy_score(strategy_file)
    print(f"Strategy score: {score}")


if __name__ == '__main__':
    main()


