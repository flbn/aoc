### Part 1:
- Input is two coloumns: `Opp``You``\n`
  - The first column is what your opponent is going to play
  - The second column, you reason, must be what you should play in response
- Winning every time would be suspicious, so the responses must have been carefully chosen.

- Scoring:
  - Total score is the sum of two scores each round.
    - One: The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
    - Two: the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    - Ex:
      - XY -> 2 for selecting Paper + 6 for winning
      - YY -> 2 for selecting Paper + 3 for tieing
      - ZY -> 2 for selecting Paper + 0 for losing

- Misc:
  - `X` for Rock, `Y` for Paper, and `Z` for Scissors

- Strat:
  - Enum can capture value:
    - X: 1
    - Y: 2
    - Z: 3
  - Another enum for the result:
    - Win: 6
    - Tie: 3
    - Loss: 0
  - Do the values reflect their ability to win?
  - Scenarios:
    - Rock (1):
      - Ties vs Rock (1)
      - Loses vs Paper (2)
      - Wins vs Scissors (3)
    - Paper (2):
      - Wins vs Rock (1)
      - Ties vs Paper (2)
      - Loses vs Scissors (3)
    - Scissors (3):
      - Loses vs Rock (1)
      - Wins vs Paper (2)
      - Ties vs Scissors (3)
    - Patterns?
      - abs = 0, tie
      - hmm, well if you go +1 (out of bounds being 3 + 1 = 1) then you meet your maker (win)
      - similarly, if you go -1 (out of bounds being 1 - 1 = 3) then you meet who you can beat
      - Ex:
        - Rock (1) - 1 = Scissors (3) and you win
        - Scissors (3) - 1 = Paper (2) and you win
        - Rock (1) + 1 = Paper (2) and you lose
        - Scissors (3) + 1 = Rock (1) and you lose

### Part 2:
- Second column:
  - how the round needs to end:
    - X: you need to lose
    - Y: you need to end the round in a draw
    - Z: you need to win
