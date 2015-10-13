#!python3.5

def binomial(n, k):
  # Since binomial(n, k) = binomial(n, n - k), we might as well use
  # the smaller k to optimize
  if n - k < k:
    k = n - k

  # Compute the coefficient
  res = 1
  for i in range(1, k + 1):
    res = res * (n - k + i)
    res = res // i

  return res

test_cases = int(input())
for i in range(0, test_cases):
  N, M = [ int(v) for v in input().split() ]
  print(binomial(N+M, M) % (pow(10, 9)+7))
