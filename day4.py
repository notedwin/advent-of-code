with open("test.txt", 'r') as f:
    boards = []
    board = []
    for index,line in enumerate(f):
        if index == 0:
            nums = line.strip().split(',')
        elif line.strip():
            board.append(line.strip().split())
        else:
            boards.append(board)
            board = []
    boards.append(board)
    board = []



def checkbingo(board):
    for row in board:
        if row.count('X') == 5:
            return True
    return any(
        board[0][col] == 'X'
        and board[1][col] == 'X'
        and board[2][col] == 'X'
        and board[3][col] == 'X'
        and board[4][col] == 'X'
        for col in range(5)
    )
    
def getfirstBingo(nums,boards):
    for num in nums:
        for board in boards:
            for rindex,row in enumerate(board):
                for cindex,val in enumerate(row):
                    if val == num:
                        board[rindex][cindex] = 'X'
                    if checkbingo(board):
                        return board,int(num)

def getallNumbers(board):
    s = 0
    for row in board:
        for val in row:
            if val != 'X':
                s += int(val)
    return s

# part 1
# board,num = getfirstBingo(nums,boards)
# remainder = getallNumbers(board)
# print(f"sum of all remaining numbers is {remainder} and the first bingo number is {num}")

# sol = getallNumbers(board)*num
# print(f"day 4 part 1 answer is {sol}")

# part 2
while len(boards) > 1:
    board,num = getfirstBingo(nums,boards)
    boards.remove(board)
    # remove all numbers up to num
board,num = getfirstBingo(nums,boards)
remainder = getallNumbers(board)
print(f"sum of all remaining numbers is {remainder} and the first bingo number is {num}")

sol = getallNumbers(board)*num
print(f"day 4 part 2 answer is {sol}")