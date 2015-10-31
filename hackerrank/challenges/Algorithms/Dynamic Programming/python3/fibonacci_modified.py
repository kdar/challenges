#!python3.5

cache = {}

def fib(n):
  if n in cache:
    return cache[n]

  cache[n] = pow(fib(n-1), 2) + fib(n-2)
  return cache[n]

t0, t1, n = [int(x) for x in input().split(' ')]
cache[-1] = t0
cache[0] = t0
cache[1] = t1
print(fib(n-1))

# # non-recursive method:
#
# [A, B, N] = map(int, raw_input().split())
# f = [A,B]
# while len(f) < N:
#     f.append(f[-1]*f[-1] + f[-2])
# print f[-1]
