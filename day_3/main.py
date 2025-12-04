def get_joltage(string, length=12, current=""):
    if length > len(string):
        return None
    if length==0:
        return current
    if length==1:
        return current + max(string)

    for c in "987654321":
        if c in string:
            index = string.index(c)
            substr = string[index+1:]
            if (result := get_joltage(substr, length-1, current+c)) is not None:
                return result

            
file_path = 'input.txt'
lines = []
with open(file_path, 'r') as file:
    lines.append(file.readline())

for line in lines:
    print(get_joltage(line))
    break
