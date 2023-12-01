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
"""
import concurrent.futures


def sum_calibration_codes(file_path):
    file = open(file_path, "r")
    result = 0
    for line in file:
        result += get_line_numbers(line)
    return result


def get_line_numbers(line):
    with concurrent.futures.ThreadPoolExecutor(max_workers=2) as executor:
        first_number_future = executor.submit(get_first_number, line)
        last_number_future = executor.submit(get_first_number, line[::-1])
        x = first_number_future.result()
        y = last_number_future.result()
        return int(f'{x+y}')


def get_first_number(line):
    if line[0].isdigit():
        return line[0]
    return get_first_number(line[1:], )


if __name__ == "__main__":
    sum_codes = sum_calibration_codes("calibration_codes.txt")
    print(sum_codes)
