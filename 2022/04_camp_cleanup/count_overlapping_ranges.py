import argparse
import pathlib

def count_overlapping_ranges(pair_assignment_file):
    count = 0
    for line in pair_assignment_file:
        interval_a, interval_b = [interval_str.split('-', maxsplit=1) for interval_str in line.strip().split(',', maxsplit=1)]
        interval_a = [int(value) for value in interval_a]
        interval_b = [int(value) for value in interval_b]
        if (
            (interval_a[0] >= interval_b[0] and interval_a[0] <= interval_b[1]) or
            (interval_a[1] >= interval_b[0] and interval_a[1] <= interval_b[1]) or
            (interval_b[0] >= interval_a[0] and interval_b[0] <= interval_a[1]) or
            (interval_b[1] >= interval_a[0] and interval_b[1] <= interval_a[1])
        ):
            count += 1

    return count

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path to pair assignment file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as pair_assignment_file:
        count = count_overlapping_ranges(pair_assignment_file)
    print(f"Fully containing ranges count: {count}")


if __name__ == '__main__':
    main()
