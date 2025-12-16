import argparse
from dataclasses import dataclass
import functools
import pathlib


@dataclass
class Position:
    x: int
    y: int

    def move(self, x, y):
        self.x += x
        self.y += y


def saturate(val):
    return max(-1, min(1, val))


def count_visited_positions_by_tail(input_file, knot_count):
    knots = [Position(0, 0) for _ in range(knot_count)]
    motions = {
        'U': functools.partial(knots[0].move, 0, 1),
        'D': functools.partial(knots[0].move, 0, -1),
        'L': functools.partial(knots[0].move, -1, 0),
        'R': functools.partial(knots[0].move, 1, 0),
    }
    visited_positions_by_tail = set()
    for line in input_file:
        motion, count = line.rstrip().split(maxsplit=1)
        for _ in range(int(count)):
            motions[motion]()
            for i in range(1, len(knots)):
                diff_x = knots[i - 1].x - knots[i].x
                diff_y = knots[i - 1].y - knots[i].y
                if abs(diff_x) > 1 or abs(diff_y) > 1:
                    knots[i].move(saturate(diff_x), saturate(diff_y))
            visited_positions_by_tail.add((knots[-1].x, knots[-1].y))
    return len(visited_positions_by_tail)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path to challenge input file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as input_file:
        count = count_visited_positions_by_tail(input_file, 2)  # Only head and tail
        print(f"Tail visited {count} positions for a rope of 2 knots.")
    with args.file.open('r', encoding='utf-8') as input_file:
        count = count_visited_positions_by_tail(input_file, 10)
        print(f"Tail visited {count} positions for a rope of 10 knots.")


if __name__ == '__main__':
    main()
