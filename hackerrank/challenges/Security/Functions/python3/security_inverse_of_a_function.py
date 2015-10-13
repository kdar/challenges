input()
l = [int(x) for x in input().split()]
l2 = [0 for _ in l]
for (x, i) in enumerate(l):
  l2[i-1] = x+1

print("\n".join([str(x) for x in l2]))
