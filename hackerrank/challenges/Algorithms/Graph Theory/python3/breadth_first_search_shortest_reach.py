#!python3.5

from collections import deque

class Node:
  distance = -1
  visited = False
  n = -1

  def __init__(self, n):
    self.n = n

class Graph:
  adj = []
  nodes = []
  size = -1

  def __init__(self, size):
    self.size = size
    self.adj = [[] for x in range(self.size+1)]
    self.nodes = [Node(x) for x in range(self.size+1)]

  def add_edge(self, x, y):
    if y not in self.adj[x]:
      self.adj[x].append(y)
    if x not in self.adj[y]:
      self.adj[y].append(x)

  def bfs(self, s):
    visited = [False for x in range(self.size+1)]
    q = deque([])

    q.append(s)
    visited[s] = True
    self.nodes[s].distance = 0

    while len(q) > 0:
      u = q.popleft()

      for i in self.adj[u]:
        if not visited[i]:
          visited[i] = True
          q.append(i)
          self.nodes[i].distance = self.nodes[u].distance + 6

def main():
  T = int(input())

  for _ in range(T):
    N,M = [int(x) for x in input().split()]
    g = Graph(N)

    for i in range(M):
      x,y = [int(x) for x in input().split()]
      g.add_edge(x, y)

    S = int(input())

    g.bfs(S)

    for i in range(1, N+1):
      if i == S: continue
      print(g.nodes[i].distance, end=" ")
    print("")

if __name__ == "__main__":
  main()
