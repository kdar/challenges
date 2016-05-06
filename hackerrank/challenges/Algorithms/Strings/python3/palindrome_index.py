#!python3.5

def is_palindrome(s):
  for x in range(len(s)//2):
    if s[x] != s[len(s)-x-1]:
      return False
  return True

def palindrome_index(s):
  for x in range(len(s)//2):
    if s[x] != s[len(s)-x-1]:
      if is_palindrome(s[x:len(s)-x-1]):
        return len(s) - x - 1
      else:
        return x
  return -1

test_cases=input()
for i in range(int(test_cases)):
  print(palindrome_index(input().strip()))

# print(palindrome_index("aaab"))
# print(palindrome_index("abaa"))
# print(palindrome_index("aaa"))
