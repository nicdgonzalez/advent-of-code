import pathlib

# Get input from input.txt
print(__file__)
# Wrap the current file's path in a pathlib.Path object.
filepath = pathlib.Path(__file__)
print(filepath)
# Get the path to the original file, in case of symbolic links.
filepath = filepath.resolve()
# We don't want the current file, we want the directory that it's in.
filepath = filepath.parent
print(filepath)
# We don't want the path to the directory itself, we want the path to input.txt
filepath = filepath.joinpath("input.txt")
print(filepath)
# Get the contents of the file.
data = filepath.read_text()
print(data.splitlines()[0:3])

print("SOLUTION STARTS".center(80, "="))

# Solve for Advent of Code:
safe_total = 0

for line in data.splitlines():
    print(line)
    numbers = line.split(sep=" ")
    print(numbers)
    # Convert string numbers into integers
    numbers = [int(n) for n in numbers]
    print(numbers)

    position = 0
    is_increasing = numbers[0] < numbers[1]

    while position < len(numbers) - 1:  # -1 = Ignore the last number
        number = numbers[position]
        number_next = numbers[position + 1]
        print(number, "<" if is_increasing else ">", number_next, number < number_next, is_increasing)

        is_consistent = (number < number_next) == is_increasing
        print("is_consistent:", is_consistent)
        is_within_3 = (1 <= abs(number - number_next) <= 3)
        print("is_within_3:", is_within_3)
        print()

        if not (is_consistent and is_within_3):
            print("did hit the break")
            break

        position += 1
    else:
        print("incremented total")
        safe_total += 1

print("part one:", safe_total)
