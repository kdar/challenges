#!python3.5

import sys
import io

TEST = """3
3
4
7
"""

def solve(reader):
  test_cases=reader.readline()

  for i in range(int(test_cases)):
    N = pow(2, int(reader.readline().strip()))

    sum = 0
    while True:
      sum += N % 10
      N = N // 10
      if N == 0:
        break
    print(sum)

def main():
  # for testing purposes
  if len(sys.argv) == 2 and sys.argv[1] == "test":
    solve(io.StringIO(TEST))
    return
  solve(sys.stdin)

if __name__ == "__main__":
  main()
