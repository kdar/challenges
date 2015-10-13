import fractions

T=int(input())
for _ in range(T):
  a,b,x,y = [int(x) for x in input().strip().split()]

  if fractions.gcd(a,b) == fractions.gcd(x,y):
    print("YES")
  else:
    print("NO")
    
