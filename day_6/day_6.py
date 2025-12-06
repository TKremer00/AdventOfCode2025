from functools import reduce

def calcualte(input: list[str], operation: str) -> int:
    numbers = [int(x) for x in input]

    match operation:
        case '*':
            return reduce(lambda x,y: x * y, numbers)
        case '+':
            return reduce(lambda x,y: x + y, numbers)

    raise Exception('should not get here')

def part1():
    file_path = 'input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    input = input.strip().split('\n')
    operations = input[len(input)-1].split()
    input = input[:len(input)-1]
    input = [s.split() for s in input]
    line_length = max(len(row) for row in input)

    groups = []
    for col in range(line_length):
        digits = []
        for row in input:
            if col >= len(row):
                continue
    
            digits.append(row[col])
        groups.append(digits)

    if len(groups) != len(operations):
        raise Exception('There should be an operation for every group of numbers')

    total = 0
    for (i, group) in enumerate(groups):
        operation = operations[i]
        total += calcualte(group,operation)

    print(f'total sum is {total}')

def part2():
    file_path = 'input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    input = input.strip().split('\n')
    operations = input[len(input)-1].split()
    input = input[:len(input)-1]
    numbers = []

    max_column_length = max(len(row) for row in input)

    result = []
    for col in range(max_column_length):
        digits = []

        for row in input:
            if col >= len(row):
                continue
            if row[col] == ' ':
                continue
    
            digits.append(row[col])
        result.append(''.join(digits))

    groups = []
    current = []
    for item in result:
        if item == "":
            groups.append(current)
            current = []
        else:
            current.append(item)

    if current:
        groups.append(current)

    if len(groups) != len(operations):
        raise Exception('There should be an operation for every group of numbers')

    total = 0
    for (i, group) in enumerate(groups):
        operation = operations[i]
        total += calcualte(group,operation)
    
    print(f'total sum is {total}')

import time
if __name__ == '__main__':
    start = time.perf_counter()
    part1()
    end = time.perf_counter()

    elapsed_ms = (end - start) * 1000
    print(f"Elapsed time: {elapsed_ms:.3f} ms")

    start = time.perf_counter()
    part2()
    end = time.perf_counter()

    elapsed_ms = (end - start) * 1000
    print(f"Elapsed time: {elapsed_ms:.3f} ms")
