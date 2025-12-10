from collections import deque
import heapq
import concurrent.futures
from concurrent.futures import Executor

class Button:
    def __init__(self, indexes):
        mask = 0
        for i in indexes:
            mask |= (1 << i)
        self.mask = mask
        self.indexes = indexes

class Machine:
    def __init__(self, indicators: str, buttons: list[Button], joltage: list[int]) -> None:
        self._indicators_mask = sum((1 << i) for i, c in enumerate(indicators) if c == '#')
        self._buttons = buttons
        self._joltage = joltage

    def turn_machine_on(self) -> int:
        start = 0
        queue = deque([(start, 0)])
        visited = {start}

        while queue:
            state, presses = queue.popleft()

            if state == self._indicators_mask:
                return presses

            for button in self._buttons:
                next_state = state ^ button.mask
                if next_state not in visited:
                    visited.add(next_state)
                    queue.append((next_state, presses + 1))

        raise Exception("No solution found")

    def configure_joltage(self) -> int:
        target = self._joltage
        buttons = [b.indexes for b in self._buttons]

        buttons.sort(key=len, reverse=True)

        best = sum(target)
        n = len(target)

        stack = [(0, [0]*n, 0)]

        while stack:
            idx, current, presses = stack.pop()

            if presses >= best:
                continue

            if current == target:
                best = presses
                continue

            if idx == len(buttons):
                continue

            remaining = set(i for b in buttons[idx:] for i in b)
            invalid = any(
                current[i] != target[i] and i not in remaining
                for i in range(n)
            )
            if invalid:
                continue

            max_press = min(
                target[i] - current[i] for i in buttons[idx]
            )

            for k in range(max_press, -1, -1):
                next_state = current[:]
                for i in buttons[idx]:
                    next_state[i] += k

                stack.append((idx + 1, next_state, presses + k))

        return best
    

def line_to_machine(line: str) -> Machine:
    indicator_end = line.find(']') + 1
    joltage_start = line.find('{');
    indicators = line[1:indicator_end-1]
    buttons_raw = line[indicator_end:joltage_start].strip().split()
    buttons = [
        Button([int(i) for i in b[1:-1].split(',')])
        for b in buttons_raw
    ]
    joltages = line[joltage_start+1:-1]
    return Machine(indicators, buttons, [int(j) for j in joltages.split(',')] )

def part1():
    file_path = 'input.txt'

    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    lines = input.strip().split('\n')
    presses = []
    for line in lines:
        machine = line_to_machine(line)
        presses.append(machine.turn_machine_on())

    print(f'answer is = {sum(presses)}')

def get_answer_for_line(line: str) -> int:
    machine = line_to_machine(line)
    return machine.configure_joltage()


def part2():
    file_path = 'input.txt'
    input = None
    with open(file_path, 'r') as file:
        input = file.read()

    lines = input.strip().split('\n')
    total_lines = len(lines)
    presses = []

    with concurrent.futures.ThreadPoolExecutor(max_workers=11) as executor:
        futures = []

        for line in lines:
            futures.append(executor.submit(lambda: get_answer_for_line(line)))

        i = 0
        for future in concurrent.futures.as_completed(futures):
            i += 1
            print(f'{i}/{total_lines}')
            presses.append(future.result())



    print(presses)
    print(f'answer is = {sum(presses)}')

if __name__ == '__main__':
    part1()
    part2()
    pass
