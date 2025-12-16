import pathlib

from treetop_tree_house import compute_highest_scenic_score
from treetop_tree_house import count_visible_trees

def main():
    with pathlib.Path('example1.txt').open('r', encoding='utf-8') as tree_map_file:
        assert count_visible_trees(tree_map_file) == 4

    with pathlib.Path('example2.txt').open('r', encoding='utf-8') as tree_map_file:
        assert count_visible_trees(tree_map_file) == 21

    with pathlib.Path('example1.txt').open('r', encoding='utf-8') as tree_map_file:
        assert compute_highest_scenic_score(tree_map_file) == 0

    with pathlib.Path('example2.txt').open('r', encoding='utf-8') as tree_map_file:
        assert compute_highest_scenic_score(tree_map_file) == 8

    with pathlib.Path('input.txt').open('r', encoding='utf-8') as tree_map_file:
        assert compute_highest_scenic_score(tree_map_file) == 410400

if __name__ == '__main__':
    main()
