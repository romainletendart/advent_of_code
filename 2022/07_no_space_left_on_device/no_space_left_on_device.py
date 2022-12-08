import argparse
from dataclasses import dataclass
from dataclasses import field
import pathlib
from typing import Dict


@dataclass
class Entry:
    name: str


@dataclass
class DirectoryEntry(Entry):
    parent: 'DirectoryEntry'
    children: Dict[str, Entry] = field(default_factory=dict)

    @property
    def size(self):
        return sum(child.size for child in self.children.values())

    def iter_dir_sizes(self):
        yield self.size
        for child in self.children.values():
            if isinstance(child, DirectoryEntry):
                for size in child.iter_dir_sizes():
                    yield size


class RootDirectoryEntry(DirectoryEntry):
    def __init__(self, **kwargs):
        kwargs['parent'] = self
        super().__init__(**kwargs)


@dataclass
class FileEntry(Entry):
    size: int


class FileSystem:
    def __init__(self):
        self.root = RootDirectoryEntry(name='/')

    def populate_from(self, terminal_output_file):
        cwd = None
        command = None
        for line in terminal_output_file:
            line = line.rstrip()
            tokens = line.split()
            if tokens[0] == '$':
                command = tokens[1]
                if command == 'cd':
                    cd_arg = tokens[2]
                    if cd_arg == '/':
                        cwd = self.root
                    elif cd_arg == '..':
                        cwd = cwd.parent
                    else:
                        cwd = cwd.children[cd_arg]
            else:
                if command == 'ls':
                    entry_name = tokens[1]
                    if tokens[0] == 'dir':
                        cwd.children.setdefault(entry_name, DirectoryEntry(name=entry_name, parent=cwd))
                    else:
                        cwd.children.setdefault(entry_name, FileEntry(name=entry_name, size=int(tokens[0])))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="terminal output file", type=pathlib.Path)
    args = parser.parse_args()

    fs = FileSystem()
    with args.file.open('r', encoding='utf-8') as terminal_output_file:
        fs.populate_from(terminal_output_file)
    max_dir_size = 100000
    total_size_sum = sum(size for size in fs.root.iter_dir_sizes() if size <= max_dir_size)
    print(f"Sum of directory total sizes (max size per dir={max_dir_size}): {total_size_sum}")

    total_size = 70000000
    update_size = 30000000
    free_size = total_size - fs.root.size
    assert free_size >= 0
    missing_size = update_size - free_size
    if missing_size > 0:
        smallest_freeable_size = min(size for size in fs.root.iter_dir_sizes() if size >= missing_size)
        print(f"Missing {missing_size}, can free {smallest_freeable_size}")


if __name__ == '__main__':
    main()
