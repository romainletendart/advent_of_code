import pathlib

from find_packet_start import find_packet_start
from find_message_start import find_message_start


def main():
    with pathlib.Path('example1.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_packet_start(datastream_file) == 7
    with pathlib.Path('example2.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_packet_start(datastream_file) == 5
    with pathlib.Path('example3.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_packet_start(datastream_file) == 6
    with pathlib.Path('example4.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_packet_start(datastream_file) == 10
    with pathlib.Path('example5.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_packet_start(datastream_file) == 11

    with pathlib.Path('example1.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_message_start(datastream_file) == 19
    with pathlib.Path('example2.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_message_start(datastream_file) == 23
    with pathlib.Path('example3.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_message_start(datastream_file) == 23
    with pathlib.Path('example4.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_message_start(datastream_file) == 29
    with pathlib.Path('example5.txt').open('r', encoding='utf-8') as datastream_file:
        assert find_message_start(datastream_file) == 26

if __name__ == '__main__':
    main()
