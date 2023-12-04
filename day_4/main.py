import pathlib
import re

# Read the input file into a list
input_loc = pathlib.Path(__file__).parent.resolve().__str__() + "\\resources\\input.txt"

with open(input_loc) as f:
    lines = f.read().splitlines()

total_points = 0
total_scratchcards = 0
scratchcard_lookup = {}

# Part 1
for i in range(lines.__len__()):
    line = lines[i]
    card = line.split(":")[0]
    card_num = card.split(" ")[1]
    round = line.split(":")[1]
    winning_numbers = re.split(r"\s+", round.split("|")[0].strip())
    played_numbers = re.split(r"\s+", round.split("|")[1].strip())
    print("Card " + card_num + ": Winning numbers - ", winning_numbers, " : Played numbers - ", played_numbers)
    matched_wins = 0
    for number in winning_numbers:
        if number in played_numbers:
            matched_wins += 1
    
    if matched_wins > 0:
            # Calculate the total worth by performing a power operation
            total_points += 2 ** (matched_wins - 1)

            # The number of times we process the scratchcards is equal to the number of instances
            # in the lookup + this iteration
            iterations = 1
            if i in scratchcard_lookup:
                iterations += scratchcard_lookup[i]

            for iteration in range(iterations):
                # Update the lookup for subsequent cards
                for card in range(matched_wins):
                    card_index = i + card + 1
                    if card_index in scratchcard_lookup:
                        scratchcard_lookup[card_index] += 1
                    else:
                        scratchcard_lookup[card_index] = 1

print("Total points: ",total_points)

for key in list(scratchcard_lookup.keys()):
    total_scratchcards += scratchcard_lookup[key]

total_scratchcards += lines.__len__()

print("Total scratchcards: ",total_scratchcards)