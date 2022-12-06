import argparse
import collections
import pathlib
import re

def get_crates_on_top(stacks_rearrangement_file):
    stacks = []
    stack_count = 0
    # Parse initial stack arrangement
    for line in stacks_rearrangement_file:
        if not line.strip():
            # We're done parsing initial stack arrangement
            break
        if stack_count == 0:
            stack_count = len(line) // 4
            stacks.extend(collections.deque() for i in range(stack_count))
        for i, stack_match in enumerate(re.findall(r'(?:( {3}\s)|\[(\w)\]\s)+?', line)):
            stack_repr = stack_match[1]
            if stack_repr:
                stacks[i].appendleft(stack_repr)
    # Parse stack movements
    for line in stacks_rearrangement_file:
        quantity, original, target = re.findall(r'move (\d+) from (\d+) to (\d+)', line)[0]
        tmp_stack = collections.deque()
        for i in range(int(quantity)):
            tmp_stack.append(stacks[int(original) - 1].pop())
        for i in range(int(quantity)):
            stacks[int(target) -1].append(tmp_stack.pop())
    return ''.join(stack.pop() for stack in stacks)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path to pair assignment file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as stacks_rearrangement_file:
        crates_on_top = get_crates_on_top(stacks_rearrangement_file)
    print(f"Crates on top: {crates_on_top}")


if __name__ == '__main__':
    main()
