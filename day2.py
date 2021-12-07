# part 1
coord = [0,0]
with open("day2.txt", 'r') as f:
    commands = [line.split() for line in f]

for command in commands:
    if command[0] == "forward":
        coord[0] += int(command[1])
    elif command[0] == "down":
        coord[1] += int(command[1])
    elif command[0] == "up":
        coord[1] -= int(command[1])

print(f"part 1: {coord[0] * coord[1]}")


# part 2
coord = {"x":0,"y":0,"aim":0}

for command in commands:
    if command[0] == "forward":
        coord["x"] += int(command[1])
        coord["y"] += (coord["aim"]*int(command[1]))
    elif command[0] == "down":
        coord["aim"] += int(command[1])
    elif command[0] == "up":
        coord["aim"] -= int(command[1])
        
print(f"part 2: {coord['x'] * coord['y']}")