# part 1
with open("day1.txt", 'r') as f:
    lines = []
    greater = 0
    for line in f:
        if lines and int(line) > lines[-1]:
            greater += 1
        lines.append(int(line))

print(greater)

# part 2
# list comprehension ftw
with open("day1.txt", 'r') as f:
    lines = [int(line) for line in f]
    windows = [sum(lines[index:index+3]) for index in range(len(lines)- 2)]
    greater = sum(windows[index] > windows[index -1] for index in range(1,len(windows)))

print(greater)
