class Shape:
    def __init__(self, index, grid: list[list[chr]]):
        self.index = index
        self.grid = grid

    def area(self) -> int:
        area = 0
        for row in self.grid:
            for column in row:
                if column == '#':
                    area += 1
        return area

class Region:
    def __init__(self, width: int, height: int, shapes: list[int]):
        self.width = width
        self.height = height
        self.shapes = shapes

    def area(self) -> int:
        return self.width * self.height

def parse_input(input: str) -> (list[Region], list[Shape]):
    lines = input.strip().split('\n')
    shapes = []
    regions = []

    i = 0
    while i < len(lines):
        line = lines[i]

        if line == '':
            i+=1
            continue

        if line[0].isdigit() and line[1] == ':':
            shape_index = int(line[0])
            shape_grid = [list(x) for x in lines[i+1:i+4]]
            shapes.append(Shape(shape_index, shape_grid))
            i += 4
        else:
            parts = line.split(':', 1)
            size = parts[0].split('x', 1)
            width = int(size[0])
            heigth = int(size[1])
            region_shapes = [int(x) for x in parts[1].strip().split(' ')]
            regions.append(Region(width, heigth, region_shapes))

        i+=1
    return (regions, shapes)


def do_all_packages_fit(region: Region, shapes: list[Shape]) -> bool:
    total_present_area = 0
    for (i, count) in enumerate(region.shapes):
        total_present_area += count * shapes[i].area()

    return region.area() > total_present_area

def part1():
    file_path = 'input.txt'

    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    (regions, shapes) = parse_input(input)

    fit_count = 0
    for region in regions:
        if do_all_packages_fit(region, shapes):
            fit_count += 1

    print(f'answer {fit_count}')


def part2():
    file_path = 'test_input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    (regions, shapes) = parse_input(input)

if __name__ == '__main__':
    part1()
