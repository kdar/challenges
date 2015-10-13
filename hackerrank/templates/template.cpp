#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <sstream>
#include <string.h>
using namespace std;

char TEST[] = "";

void solve(istream &input) {
  // int test_cases = 0;
  // string line;
  // input >> test_cases;
  // for (; test_cases; test_cases--) {
  //   input >> line;
  // }
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
