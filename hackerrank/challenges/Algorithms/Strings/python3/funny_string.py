#!python3.5

import sys
import io

TEST = """2
acxz
bcxz
"""

def solve(reader):
  test_cases=reader.readline()
  for i in range(int(test_cases)):
    line = reader.readline().strip()
    rev = line[::-1]
    funny = True
    for i in range(1, len(line)):
      if abs(ord(line[i])-ord(line[i-1])) != abs(ord(line[len(line)-i-1])-ord(line[len(line)-i])):
        funny = False
        break
    if funny:
      print("Funny")
    else:
      print("Not Funny")

def main():
  # for testing purposes
  if len(sys.argv) == 2 and sys.argv[1] == "test":
    solve(io.StringIO(TEST))
    return
  solve(sys.stdin)

if __name__ == "__main__":
  main()
