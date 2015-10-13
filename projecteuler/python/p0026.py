def main():
    Max = 0
    Solution = {0:0}

    for d in reversed(range(2,1000)):
        Dividen = 1
        Count = 0
        if IsRepeating(d):
            while True:
                Dividen *= 10
                Digit = Dividen // d
                Dividen = Dividen - Digit*d
                Count += 1
                if Dividen == 1:
                    break
        if Count > Max:
            del Solution[Max]
            Max = Count
            Solution[Max] = d

    print(Solution)

def IsRepeating(Number):
    Factors = []
    Check = 2
    while Number > 1:
        if Number % Check == 0:
            Factors.append(Check)
            Number = Number / Check
            Check = 1
        Check += 1

    if 2 in Factors or 5 in Factors:
        return False
    else:
        return True

main()
