import argparse
import pathlib


def find_message_start(datastream_file):
    datastream = datastream_file.readline().rstrip()
    marker_length = 14
    first_marker = marker_length
    while len(set(datastream[first_marker - marker_length:first_marker])) != marker_length:
        first_marker += 1
    return first_marker


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', help="Path datastream file", type=pathlib.Path)
    args = parser.parse_args()

    with args.file.open('r', encoding='utf-8') as datastream_file:
        first_marker = find_message_start(datastream_file)
    print(f"First marker: {first_marker}")


if __name__ == '__main__':
    main()
