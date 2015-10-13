#!python3.5

import sys
import io

TEST = """5
0 0
m----
-----
---p-
-----
-----"""

def princessLoc(grid):
  for y in range(len(grid)):
    for x in range(len(grid[y])):
      if grid[y][x] == "p":
        return {"x": x, "y": y}
  return None

def nextMove(n,r,c,grid):
  p_pos = princessLoc(grid)

  y_diff = r - p_pos["y"]
  x_diff = c - p_pos["x"]
  #print(y_diff, x_diff)
  if x_diff == 0 or (y_diff != 0 and abs(y_diff) < abs(x_diff)):
    return "UP" if y_diff > 0 else "DOWN"
  else:
    return "LEFT" if x_diff > 0 else "RIGHT"

def solve(reader):
  n = int(reader.readline())
  r,c = [int(i) for i in reader.readline().strip().split()]
  grid = []
  for i in range(0, n):
    grid.append(reader.readline())

  print(nextMove(n,r,c,grid))

def main():
  # for testing purposes
  if len(sys.argv) == 2 and sys.argv[1] == "test":
    solve(io.StringIO(TEST))
    return
  solve(sys.stdin)

if __name__ == "__main__":
  main()
