from dataclasses import dataclass
from functools import reduce
from pprint import pprint
import math

@dataclass(eq=True, frozen=True)
class Coordinate:
    x: int
    y: int
    z: int

    def calculate_distance(self, other: "Coordinate") -> float:
        result = pow(self.x - other.x, 2) + pow(self.y - other.y, 2) + pow(self.z - other.z, 2) 
        return math.sqrt(result)

@dataclass(eq=False, frozen=True)
class DistanceCoordinate:
    a: Coordinate
    b: Coordinate

    def get_distance(self) -> float:
        return self.a.calculate_distance(self.b)

    def __eq__(self, other):
        if isinstance(other, Coordinate):
            return other == self.a or other == self.b

        return {self.a, self.b} == {other.a, other.b}

@dataclass(eq=True, frozen=True)
class Circuit:
    coordinates: set[Coordinate]

    def add_coordinate(self, coordiante: "Coordinate"):
        self.coordinates.add(coordiante)

    def add_coordinates(self, coordiantes: set[Coordinate]):
        self.coordinates.update(coordiantes)

    def get_total_coordinates(self) -> int:
        coordinates = set()
        for c in self.coordinates:
            coordinates.add(c)
        return len(coordinates)

    
    def contains_coordinate(self, coord: Coordinate) -> bool:
        return coord in self.coordinates

def read_cooridinates(input: str) -> list[Coordinate]:
    coordinates = []
    lines = input.strip().split('\n')
    for line in lines:
        axes = line.split(',')
        coordinates.append(Coordinate(int(axes[0]), int(axes[1]), int(axes[2])))

    return coordinates

def to_distance_coordiantes(coordinates: list[Coordinate]) -> list[DistanceCoordinate]:
    distances = []

    for i in range(len(coordinates) - 1):
        for j in range(i+1, len(coordinates)):
            a = coordinates[i]
            b = coordinates[j]
            distances.append(DistanceCoordinate(a,b))
    return distances

def group_coordinates(distances: list[DistanceCoordinate]) -> list[Circuit]:
    groups = []
    connections = 0
    find_group = lambda c: next((g for g in groups if  g.contains_coordinate(c)), None)
    distances = sorted(distances, key=lambda x: x.get_distance())

    for distance in distances[:10]:
        connections += 1

        group1Index = -1
        group2Index = -1
        for i in range(len(groups)):
            if groups[i].contains_coordinate(distance.a):
                group1Index = i

            if groups[i].contains_coordinate(distance.b):
                group2Index = i

        if group1Index == -1 and group2Index == -1:
            group = Circuit(set())
            group.add_coordinate(distance.a)
            group.add_coordinate(distance.b)
            groups.append(group)
        elif group1Index != -1 and group2Index == -1:
            groups[group1Index].add_coordinate(distance.b)
        elif group1Index == -1 and group2Index != -1:
            groups[group2Index].add_coordinate(distance.a)
        elif group1Index != group2Index:
            groups[group1Index].add_coordinates(groups[group2Index].coordinates)
            groups.pop(group2Index)

    return groups

def part1():
    file_path = 'input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    coordinates = read_cooridinates(input)
    distances = to_distance_coordiantes(coordinates)
    groups = group_coordinates(distances)
    test = [x.get_total_coordinates() for x in groups]
    total_sum = reduce(lambda x,y: x * y, test[:3])
    print(total_sum)

def part2():
    file_path = 'test_input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()


import time
if __name__ == '__main__':
    start = time.perf_counter()
    part1()
    end = time.perf_counter()

    elapsed_ms = (end - start) * 1000
    print(f"Elapsed time: {elapsed_ms:.3f} ms")

    # start = time.perf_counter()
    # part2()
    # end = time.perf_counter()

    # elapsed_ms = (end - start) * 1000
    # print(f"Elapsed time: {elapsed_ms:.3f} ms")
