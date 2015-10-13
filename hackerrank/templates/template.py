#!python3.5

import sys
import io

TEST = """"""

def solve(reader):
  # test_cases=reader.readline()
  # for i in range(int(test_cases)):
  #   a, b, c = [ int(v) for v in reader.readline().strip().split() ]
  pass

def main():
  # for testing purposes
  if len(sys.argv) == 2 and sys.argv[1] == "test":
    solve(io.StringIO(TEST))
    return
  solve(sys.stdin)

if __name__ == "__main__":
  main()
