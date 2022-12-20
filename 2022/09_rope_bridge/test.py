import pathlib

from rope_bridge import count_visited_positions_by_tail

def main():
    with pathlib.Path('example.txt').open('r', encoding='utf-8') as challenge_input_file:
        assert count_visited_positions_by_tail(challenge_input_file, 2) == 13

    with pathlib.Path('example.txt').open('r', encoding='utf-8') as challenge_input_file:
        assert count_visited_positions_by_tail(challenge_input_file, 10) == 1

    with pathlib.Path('example2.txt').open('r', encoding='utf-8') as challenge_input_file:
        assert count_visited_positions_by_tail(challenge_input_file, 2) == 88

    with pathlib.Path('example2.txt').open('r', encoding='utf-8') as challenge_input_file:
        assert count_visited_positions_by_tail(challenge_input_file, 10) == 36

    with pathlib.Path('input.txt').open('r', encoding='utf-8') as challenge_input_file:
        assert count_visited_positions_by_tail(challenge_input_file, 2) == 5930

    with pathlib.Path('input.txt').open('r', encoding='utf-8') as challenge_input_file:
        assert count_visited_positions_by_tail(challenge_input_file, 10) == 2443

if __name__ == '__main__':
    main()
