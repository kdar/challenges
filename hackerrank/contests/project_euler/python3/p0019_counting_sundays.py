#!python3.5

import sys
import io
import time

TEST = """5
1900 1 1
1910 1 1
2000 1 1
2020 1 1
2015 2 1
2015 3 1
1900 1 1
1900 7 1
2015 2 1
2014 12 4
"""

def wd(year, month, day):
  weekdayname = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
  JND = day                                                      \
        + ((153 * (month + 12 * ((14 - month) // 12) - 3) + 2) // 5) \
        + (365 * (year + 4800 - ((14 - month) // 12)))              \
        + ((year + 4800 - ((14 - month) // 12)) // 4)                \
        - ((year + 4800 - ((14 - month) // 12)) // 100)              \
        + ((year + 4800 - ((14 - month) // 12)) // 400)              \
        - 32045;
  return weekdayname[JND % 7]

def months_start_range(year1, month1, day1, year2, month2, day2):
  if year1 > year2:
    year1, month1, day1, year2, month2, day2 = (year2, month2, day2, year1, month1, day1)

  if day1 > 1:
    month1 += 1
    if month1 > 12:
      year1 += 1
      month1 = 1

  total = 0
  for year in range(year1, year2 + 1):
    month_range = range(1, 13)
    if year == year1 and year == year2:
      month_range = range(month1, month2+1)
    elif year == year1:
      month_range = range(month1, 13)
    elif year == year2:
      month_range = range(1, month2+1)

    for month in month_range:
      if wd(year, month, 1) == "Sunday":
        total += 1
  return total

def solve(reader):
  test_cases=reader.readline()

  for i in range(int(test_cases)):
    year1, month1, day1 = [ int(v) for v in reader.readline().split() ]
    year2, month2, day2 = [ int(v) for v in reader.readline().split() ]

    print(months_start_range(year1, month1, day1, year2, month2, day2))

def main():
  # for testing purposes
  if len(sys.argv) == 2 and sys.argv[1] == "test":
    solve(io.StringIO(TEST))
    return
  solve(sys.stdin)

if __name__ == "__main__":
  main()
