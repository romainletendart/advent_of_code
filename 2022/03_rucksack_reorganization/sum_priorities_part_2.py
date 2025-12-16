import argparse
import pathlib


def sum_priorities(rucksack_file):
    priority_sum = 0
    common_items = set()
    for i, line in enumerate(rucksack_file, start=1):
        line = line.strip()
        if common_items:
            common_items &= set(line)
        else:
            common_items = set(line)
        if i % 3 == 0:
            assert len(common_items) == 1
            badge = common_items.pop()
            if badge.islower():
                priority_sum += ord(badge) - ord('a') + 1
            else:
                priority_sum += ord(badge) - ord('A') + 27
    return priority_sum


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path to rucksack file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as rucksack_file:
        priority_sum = sum_priorities(rucksack_file)
    print(f"Priority sum: {priority_sum}")


if __name__ == '__main__':
    main()
