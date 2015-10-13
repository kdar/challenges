#!python3.5

import sys
import io
import math
import functools

TEST = """3
3
4
5000"""

# def fib(n):
#   inverseSqrt5 = 0.44721359549995793928183473374626
#   phi = 1.6180339887498948482045868343656
#   return int(math.floor(math.pow(phi, n) * inverseSqrt5 + 0.5))

def solve(reader):
  test_cases=reader.readline()

  for i in range(int(test_cases)):
    N = int(reader.readline().strip())

    #v = math.ceil((N - 1 + math.log10(math.sqrt(5))) / math.log10((1+math.sqrt(5))/2))

    # the nth fib number is ceil(phi^n / sqrt(5)).
    # Saying a number contains N digits is the same as saying it's
    # greater than 10^(N-1)
    # So we need phi^n/sqrt(5) > 10^(N-1)
    # n * log10(phi) - log10(5)/2 > (N-1) * log10(10)
    # n * log10(phi) > (N-1) * log10(10) + log10(5)/2
    # n > ((N-1) * log10(10) + log10(5) / 2) / log10(phi)
    phi = 1.6180339887498948482045868343656
    v = math.ceil((N-1 * math.log10(10) + math.log10(5) / 2) / math.log10(phi))
    print(v)

def main():
  # for testing purposes
  if len(sys.argv) == 2 and sys.argv[1] == "test":
    solve(io.StringIO(TEST))
    return
  solve(sys.stdin)

if __name__ == "__main__":
  main()
