import argparse
import pathlib


def sum_priorities(rucksack_file):
    priority_sum = 0
    for line in rucksack_file:
        line = line.strip()
        first_compartment = set(line[0:len(line) // 2])
        second_compartment = set(line[len(line) // 2:])
        items_in_both = first_compartment & second_compartment
        for item in items_in_both:
            if item.islower():
                priority_sum += ord(item) - ord('a') + 1
            else:
                priority_sum += ord(item) - ord('A') + 27
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
