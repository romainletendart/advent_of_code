import argparse
import pathlib


def find_packet_start(datastream_file):
    datastream = datastream_file.readline().rstrip()
    first_marker = 4
    while len(set(datastream[first_marker - 4:first_marker])) != 4:
        first_marker += 1
    return first_marker


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path datastream file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as datastream_file:
        first_marker = find_packet_start(datastream_file)
    print(f"First marker: {first_marker}")


if __name__ == '__main__':
    main()
