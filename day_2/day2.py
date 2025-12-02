from typing import Union

def parse_id_range(id_range: str) -> tuple[int, int]:
    parts = id_range.split('-', 1)
    if len(parts) != 2:
        raise Exception(f'invalid input for parsing id range {id_range}')
    (part1, part2) = (int(parts[0]), int(parts[1]))
    return (part1, part2)


def get_invalid_ids_part1(start_id, end_id) -> list[int]:
    duplicate_numbers = []
    for i in range(start_id, end_id+1):
        number = str(i)
        if len(number) % 2 != 0:
            continue

        center = int(len(number) / 2)
        if number[center:] != number[:center]:
            continue

        duplicate_numbers.append(int(number))

    return duplicate_numbers

def number_has_repeats(number: str) -> bool:
    length_of_number = len(number)
    center = int(length_of_number / 2)

    for size in range(1, center + 1):
        chunk = number[:size]
        # Make chunk to full length of number, so if input has length 5 and chunk 1, repeat chunk until length is equal to 5
        repeated_chunk = chunk * (int(length_of_number / size))
        if repeated_chunk == number:
            return True
        
    return False
    
def get_invalid_ids_part2(start_id, end_id) -> list[int]:
    duplicate_numbers = []
    for i in range(start_id, end_id+1):
        number = str(i)
        if number_has_repeats(number):
            duplicate_numbers.append(int(number))

    return duplicate_numbers

def part1():
    file_path = 'input.txt'

    input = None
    with open(file_path, 'r') as file:
        input = file.read()
    id_ranges = input.split(',')
    
    answer = 0
    for id_range in id_ranges:
        (start_id, end_id) = parse_id_range(id_range)
        invalid_ids = get_invalid_ids_part1(start_id, end_id)
        answer += sum(invalid_ids)
    print(answer)

def part2():
    file_path = 'input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()
    id_ranges = input.split(',')
    
    answer = 0
    for id_range in id_ranges:
        (start_id, end_id) = parse_id_range(id_range)
        invalid_ids = get_invalid_ids_part2(start_id, end_id)
        answer += sum(invalid_ids)
    print(answer)

if __name__ == '__main__':
    part2()
    pass
