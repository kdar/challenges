import math

def encode(input):
    input = "".join([x for x in input.lower() if x.isalnum()])
    cols = int(math.ceil(math.sqrt(len(input))))
    grid = ["" for x in range(cols)]
    for (i, x) in enumerate(input):
        grid[i%cols] += x
    return " ".join(grid)
