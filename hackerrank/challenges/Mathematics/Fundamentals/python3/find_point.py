#!python3.5

#qx-px, qy-py = (x-qx, y-qy)
#x = qx+qx-px;
#y = qy+qy-py;

test_cases = int(input())
for _ in range(test_cases):
  px, py, qx, qy = [int(x) for x in input().strip().split()]
  print("{0} {1}".format(qx+qx-px, qy+qy-py))
