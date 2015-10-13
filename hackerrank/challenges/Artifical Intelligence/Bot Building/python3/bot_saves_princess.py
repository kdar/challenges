#!python3.5

def displayPathtoPrincess(n,grid):
  p_pos = {}
  m_pos = {}

  for y in range(len(grid)):
    for x in range(len(grid[y])):
      if grid[y][x] == "m":
        m_pos["x"] = x
        m_pos["y"] = y
      elif grid[y][x] == "p":
        p_pos["x"] = x
        p_pos["y"] = y

  move_vert = "UP"
  move_horz = "LEFT"
  if p_pos["y"] > m_pos["y"]:
    move_vert = "DOWN"
  if p_pos["x"] > m_pos["x"]:
    move_horz = "RIGHT"

  print((move_horz+"\n") * abs(m_pos["x"] - p_pos["x"]), end="")
  print((move_vert+"\n") * abs(m_pos["y"] - p_pos["y"]), end="")


m = int(input())
grid = []
for i in range(0, m):
    grid.append(input().strip())

displayPathtoPrincess(m,grid)
