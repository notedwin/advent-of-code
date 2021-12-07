with open("day3.txt", 'r') as f:
    bits = [list(line.strip()) for line in f]

gamma = ""
epsilon = ""

for index, _ in enumerate(bits[0]):
    count = sum(line[index] == '1' for line in bits)
    state = bool(count > len(bits) / 2)
    #print(index, count, state)
    gamma += '1' if state else '0'
    epsilon += '0' if state else '1'



print(f"Solution part 1: { int(gamma, 2) * int(epsilon, 2) }")

# create a copy of bits to work with
com = bits[::]
for index in range(len(com[0])):
    if len(com) == 1:
        num = com[:index+1]
        break
    else:
        common = '1' if sum(line[index] == '1' for line in com) >= len(com) / 2 else '0'
        com = [ line for line in com if line[index] == common ]

oxygen = int("".join(com[0]),2)

com = bits[::]
for index in range(len(com[0])):
    if len(com) == 1:
        num = com[:index+1]
        break
    else:
        common = '1' if sum(line[index] == '1' for line in com) >= len(com) / 2 else '0'
        com = [ line for line in com if line[index] != common ]

co2 = int("".join(com[0]),2)

print(f"Solution part 2: { oxygen * co2 }\n{ oxygen }, { co2 }")