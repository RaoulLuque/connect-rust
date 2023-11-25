# To not track changes of this file in your git: git update-index --assume-unchanged encoding_to_game_board.py
# To revert the above: git update-index --no-assume-unchanged encoding_to_game_board.py

# Enter the gamestate of the board to be decoded here
encoded_gamestate = 0

if __name__ == '__main__':
    res = ""
    row_counter = 1
    column_counter = 1

    for i in range(6):
        for j in range(7):
            if 1 & encoded_gamestate == 1:
                res += "B "

            elif 2 & encoded_gamestate == 2:
                res += "R "

            else:
                res += "O "

            encoded_gamestate //= 4
        res += "\n"

    print("Decoding of given gamestate is:")
    print(res)

            

