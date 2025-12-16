import argparse
import pathlib


def count_most_calories(calories_file):
    most_calories = 0
    calories = 0
    for line in calories_file:
        try:
            calories += int(line)
        except ValueError:
            calories = 0
            continue
        if calories > most_calories:
            most_calories = calories
    return most_calories


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path to file listing calories", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as calories_file:
        most_calories = count_most_calories(calories_file)
    print(f"Most calories: {most_calories}")


if __name__ == '__main__':
    main()
