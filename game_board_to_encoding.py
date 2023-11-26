# To not track changes of this file in your git: git update-index --assume-unchanged game_board_to_encoding.py
# To revert the above: git update-index --no-assume-unchanged game_board_to_encoding.py

# Enter the gamestate of the board to be encoded here
game_board ="""
O O O O O O O
O O O O O O O
O O O O O O O
O O O O O O O
O O O O O O O
O O O O O O O
"""

if __name__ == '__main__':
    res = 0
    row_counter = 1
    column_counter = 1

    # Loop through the string and add encoding number (see gamestate_helpers.rs) to res if current 
    # char is valid Letter
    for i in range(len(game_board)):
        was_letter = True

        # Check if current char is valid letter
        match game_board[i]:
            case 'O':
                res += 0

            case 'B':
                res += pow(2, 14 * (row_counter - 1) + 2 * (column_counter - 1))

            case 'R':
                res += pow(2, 14 * (row_counter - 1) + 2 * (column_counter - 1)) * 2

            case other:
                was_letter = False
            
        if(was_letter):
            if column_counter == 7:
                column_counter = 1
                row_counter += 1
            else:
                column_counter += 1

    print("Encoding of given gamestate is:")
    print(res)

            

