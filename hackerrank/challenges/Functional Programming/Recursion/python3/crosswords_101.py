#!python3.5


from enum import Enum

INPUT="""+-++++++++
+-++++++++
+-++++++++
+-----++++
+-+++-++++
+-+++-++++
+++++-++++
++------++
+++++-++++
+++++-++++
LONDON;DELHI;ICELAND;ANKARA"""

# 6 -> (0,1), (1,1), (2,1), (3,1), (4,1), (5,1)
# 6 -> (7,2), (7,3), (7,4), (7,5), (7,6), (7,7)
# 7 -> (3,5), (3,6), (4,7), (3,8), (3,9), (3,10), (3,11)
# 5 -> (3,1), (3,2), (3,3), (3,4), (3,5)


Direction_down = 0
Direction_right = 1

def checkWord (testWord,dictionary):
  words = []
  nonBlanks = len(testWord)-testWord.count(' ')
  for word in dictionary:
    incLetter = 0
    incMatch = 0
    if len (word) == len (testWord):
      for letter in testWord:
        if letter == word[incLetter]:
          incMatch += 1
        incLetter += 1
      if incMatch == nonBlanks:
        words.append(word)
  return words

def positions(grid):
  pos = []
  for y in range(len(grid)):
    for x in range(len(grid[y])):
      if grid[y][x] == '-':
        pos.append((y,x,Direction_right))
        pos.append((y,x,Direction_down))
  return pos

def stripes(pos):
  l = list(filter(
    lambda i: not (i[0]-1+i[2], i[1]-i[2], i[2]) in pos and (i[0]+1-i[2], i[1]+i[2], i[2]) in pos,
    pos
  ))

  ret = {}

  for i in l:
    if i[2] == Direction_down:
      y = i[0]
      while (y, i[1], i[2]) in pos:
        y+=1
      ret.setdefault(y-i[0], []).append(i)
    elif i[2] == Direction_right:
      x = i[1]
      while (i[0], x, i[2]) in pos:
        x+=1
      ret.setdefault(x-i[1], []).append(i)
  return ret


#print(checkWord ("   A", ["LONDON","DELHI","ICELAND","ANKARA", "CACA", "MACA"]))

lines = INPUT.split("\n")
grid = lines[:-1]
print(stripes(positions(grid)))

words = lines[-1].split(";")
print(words)
