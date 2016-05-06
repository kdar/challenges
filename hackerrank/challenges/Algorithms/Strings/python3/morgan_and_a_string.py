#!python3.5
# https://www.hackerrank.com/challenges/morgan-and-a-string

for _ in range(int(input())):
  a = input()
  b = input()

  a += 'z'
  b += 'z'

  la = len(a)
  lb = len(b)

  i,j = 0,0
  lex = ""

  while i < la and j < lb:
    if a[i:] < b[j:]:
      lex += a[i]
      i += 1

      # optimization to just keep copying from a
      # if it's the same character we just copied
      tmp = a[i-1]
      while i < la and a[i] == tmp:
        lex += a[i]
        i += 1
    else:
      lex += b[j]
      j += 1

      # optimization to just keep copying from b
      # if it's the same character we just copied
      tmp = b[j-1]
      while j < lb and b[j] == tmp:
        lex += b[j]
        j += 1

  for k in range(i,la):
    lex += a[k]
  for k in range(j,lb):
    lex += b[k]

  print(lex[:-2])
