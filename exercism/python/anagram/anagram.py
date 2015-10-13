from collections import Counter

def detect_anagrams(input, list):
    d = Counter(input.lower())
    return [x for x in list if input.lower() != x.lower() and Counter(x.lower()) == d]
