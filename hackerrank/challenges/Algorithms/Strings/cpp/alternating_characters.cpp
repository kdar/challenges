#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <sstream>
#include <string.h>
using namespace std;

char TEST[] = "5\n"
"AAAA\n"
"BBBBB\n"
"ABABABAB\n"
"BABABA\n"
"AAABBB\n";

void solve(istream &input) {
  int test_cases = 0;
  string line;

  input >> test_cases;

  for (; test_cases; test_cases--) {
    int delete_count = 0;
    input >> line;
    for (int i = 1; i < line.length(); i++) {
      if (line[i-1] == line[i]) {
        delete_count++;
      }
    }
    cout << delete_count << endl;
  }
}

int main(int argc, char *argv[]) {
  if (argc == 2 && strcmp(argv[1], "test") == 0) {
    istringstream is(TEST);
    solve(is);
  } else {
    solve(cin);
  }

  return 0;
}
