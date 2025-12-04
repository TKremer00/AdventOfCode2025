from typing import Union


def count_roles(input: list[list[str]], should_retry = False) -> int:
    positions = [[-1,-1],[0,-1],[1,-1],[-1,0],[1,0],[-1,1],[0,1],[1,1]]


    rolles_to_move_total = 0
    rolles_to_move = 1
    while rolles_to_move > 0:
        rolles_to_move = 0
        for x in range(len(input)):
            for y in range(len(input[x])):

                if input[x][y] == '.':
                    continue

                found_items = 0
                for position in positions:
                    if x + position[0] < 0:
                        continue
                    if y + position[1] < 0:
                        continue
                    if x + position[0] > len(input):
                        continue
                    if y + position[1] >= len(input[x + position[0]]):
                        continue


                    if input[x+position[0]][y+position[1]] == '@':
                        found_items += 1

                if found_items < 4:
                    input[x] = input[x][:y] + '.' + input[x][y+1:]
                    rolles_to_move += 1
        rolles_to_move_total += rolles_to_move
        if not should_retry:
            break

    return rolles_to_move_total

def part1():
    file_path = 'input.txt'

    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    input = input.split('\n')

    roles = count_roles(input)
    print(roles)

def part2():
    pass
    file_path = 'input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()
    input = input.split('\n')

    roles = count_roles(input, True)
    print(roles)

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

    pass
