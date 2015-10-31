#!python3.5

def full_of_colors(seq):
  r = 0
  g = 0
  y = 0
  b = 0

  for i in seq:
    if i == "R":
      r+=1
    elif i == "G":
      g+=1
    elif i == "Y":
      y+=1
    elif i == "B":
      b+=1

    if abs(r-g) > 1 or abs(y-b) > 1:
      return False

  return r-g == 0 and y-b == 0

print(full_of_colors("RGGR"))
print(full_of_colors("RYBG"))
print(full_of_colors("RYRB"))
print(full_of_colors("YGYGRBRB"))
