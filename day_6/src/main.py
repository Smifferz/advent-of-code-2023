import pathlib
import re
import math
from functools import reduce

# Read the input file into a list
input_loc = pathlib.Path(__file__).parent.resolve().__str__() + "/../resources/input.txt"

with open(input_loc) as f:
    lines = f.read().splitlines()

# Part 1
# times_string = lines[0].split(":")[1].strip()
# times = re.split(r"\s+", times_string)
# distances_string = lines[1].split(":")[1].strip()
# distances = re.split(r"\s+", distances_string)

# Part 2
times_string = lines[0].split(":")[1].strip()
times = re.sub(r"\s+", "", times_string).split(" ")
distances_string = lines[1].split(":")[1].strip()
# distances = re.split(r"\s+", distances_string)
distances = re.sub(r"\s+", "", distances_string).split(" ")

total_num_larger_distances = []

for i in range(times.__len__()):
    time = float(times[i])
    # Time is the number of milliseconds to complete the action

    # TODO: Write the batch implementation
    # Create a batch by halving the max, if the largest number in the first batch is valid
    # then check the second batch, reducing the batch size every time until we find a non-matching value
    # batch_max = time
    # batch_floor = math.floor(batch_max / 2)
    # # current_max_distance = 0
    # num_larger_distances = 0
    # if (time - batch_floor) * batch_floor > int(distances[i]):
    #     # current_max_distance = (time - batch_floor) * batch_floor
    #     num_larger_distances += 1
    #     batch_floor
    # break_loop = False
    last_greater = False
    current_greater = False
    num_larger_distances = 0
    mm_per_ms = 1
    while not(last_greater == True and current_greater == False):
        last_greater = current_greater
        current_distance = (time - mm_per_ms) * mm_per_ms 
        if current_distance > float(distances[i]):
            num_larger_distances += 1
            current_greater = True
        else:
            current_greater = False
        mm_per_ms += 1
    total_num_larger_distances.append(num_larger_distances)

print("Total larger distances: ", total_num_larger_distances)
print("Total larger distances (multiplied): ", reduce(lambda x, y: x*y, total_num_larger_distances))