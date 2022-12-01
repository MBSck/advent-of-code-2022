# NOTE: Quick and dirty in python
with open("puzzle_input.txt", "r") as f:
    elves = []
    total = 0
    for line in f:
        line = line.strip()
        if line == "":
            elves.append(total)
            total = 0
            continue
        total += int(line)

elves.sort()
elves = elves[::-1]
print(elves[0])
print(sum(elves[:3]))
