import argparse
import functools
import operator
import pathlib


def count_visible_trees(tree_map_file):
    visible_tree_count = 0
    tree_map = []
    for line in tree_map_file:
        line = line.rstrip()
        tree_map.append(list(line))
    height = len(tree_map)
    width = len(tree_map[0])
    for y in range(1, height - 1):
        for x in range(1, width - 1):
            # Look up
            j = y - 1
            while j >= 0 and tree_map[j][x] < tree_map[y][x]:
                j -= 1
            if j < 0:
                visible_tree_count += 1
                continue
            # Look down
            j = y + 1
            while j < height and tree_map[j][x] < tree_map[y][x]:
                j += 1
            if j >= height:
                visible_tree_count += 1
                continue
            # Look left
            i = x - 1
            while i >= 0 and tree_map[y][i] < tree_map[y][x]:
                i -= 1
            if i < 0:
                visible_tree_count += 1
                continue
            # Look right
            i = x + 1
            while i < width and tree_map[y][i] < tree_map[y][x]:
                i += 1
            if i >= width:
                visible_tree_count += 1
                continue

    # Count edge trees as always visible
    visible_tree_count += 2 * width + 2 * (height - 2)
    return visible_tree_count


def compute_highest_scenic_score(tree_map_file):
    highest_score = 0
    tree_map = []
    for line in tree_map_file:
        line = line.rstrip()
        tree_map.append(list(line))
    height = len(tree_map)
    width = len(tree_map[0])
    # Skip edges as one of their direction score will be 0 hence the final product also equaling 0.
    for y in range(1, height - 1):
        for x in range(1, width - 1):
            current_scores = [1, 1, 1, 1]
            # Look up
            j = y - 1
            while j > 0 and tree_map[j][x] < tree_map[y][x]:
                current_scores[0] += 1
                j -= 1
            # Look down
            j = y + 1
            while j < (height - 1) and tree_map[j][x] < tree_map[y][x]:
                current_scores[1] += 1
                j += 1
            # Look left
            i = x - 1
            while i > 0 and tree_map[y][i] < tree_map[y][x]:
                current_scores[2] += 1
                i -= 1
            # Look right
            i = x + 1
            while i < (width - 1) and tree_map[y][i] < tree_map[y][x]:
                current_scores[3] += 1
                i += 1
            highest_score = max(highest_score, functools.reduce(operator.mul, current_scores, 1))
    return highest_score


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path to tree map file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as tree_map_file:
        visible_tree_count = count_visible_trees(tree_map_file)
    print(f"Visible tree count: {visible_tree_count}")
    with args.file.open('r', encoding='utf-8') as tree_map_file:
        highest_scenic_score = compute_highest_scenic_score(tree_map_file)
    print(f"Highest scenic score: {highest_scenic_score}")


if __name__ == '__main__':
    main()
