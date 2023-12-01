"""
The newly-improved calibration document consists of lines of text;
each line originally contained a specific calibration value that
the Elves now need to recover. On each line, the calibration value
can be found by combining the first digit and the last digit (in
that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines
are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum
of all of the calibration values?

--- Part Two ---
Your calculation isn't quite right. It looks like some
of the digits are actually spelled out with letters:
one, two, three, four, five, six, seven, eight, and
nine also count as valid "digits".

Equipped with this new information, you now need to
find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42,
14, and 76. Adding these together produces 281.
"""

import concurrent.futures

dict_number = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9
}


def sum_calibration_codes(file_path):
    file = open(file_path, "r")
    result = 0
    for line in file:
        result += get_line_numbers(line)
    return result


def get_line_numbers(line):
    with concurrent.futures.ThreadPoolExecutor(max_workers=2) as executor:
        first_number_future = executor.submit(get_first_number, line, "", False)
        last_number_future = executor.submit(get_first_number, line[::-1], "", True)
        x = first_number_future.result()
        y = last_number_future.result()
        return int(f'{x+y}')


def get_first_number(line, acc, reverse):
    if line[0].isdigit():
        return line[0]
    if reverse: acc = line[0] + acc
    else: acc += line[0]
    for key in dict_number.keys():
        if key in acc:
            return str(dict_number[key])
    return get_first_number(line[1:], acc, reverse)


if __name__ == "__main__":
    sum_codes = sum_calibration_codes("calibration_codes.txt")
    print(sum_codes)
