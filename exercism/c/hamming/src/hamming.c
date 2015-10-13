int min(int a, int b) {
  if (a < b) {
    return a;
  }
  return b;
}

int compute(char *strand1, int len1, char *strand2, int len2) {
  int count = 0;
  for (int x = 0; x < min(len1, len2); x++) {
    if (strand1[x] != strand2[x]) {
      count++;
    }
  }
  return count;
}
