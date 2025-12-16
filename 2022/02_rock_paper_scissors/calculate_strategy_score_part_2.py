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
    target_results = {
        'X': Result.LOSS,
        'Y': Result.DRAW,
        'Z': Result.WIN,
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
    player_moves = {
        # (opponent, result): player
        (Move.ROCK, Result.LOSS): Move.SCISSORS,
        (Move.SCISSORS, Result.LOSS): Move.PAPER,
        (Move.PAPER, Result.LOSS): Move.ROCK,
        (Move.SCISSORS, Result.WIN): Move.ROCK,
        (Move.PAPER, Result.WIN): Move.SCISSORS,
        (Move.ROCK, Result.WIN): Move.PAPER,
    }
    score = 0
    for line in strategy_file:
        opponent, result = line.strip().split(maxsplit=1)
        opponent = opponent_moves[opponent]
        result = target_results[result]
        player = player_moves.get((opponent, result), opponent)
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


