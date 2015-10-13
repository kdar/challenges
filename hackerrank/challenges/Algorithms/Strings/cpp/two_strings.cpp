#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <sstream>
#include <string.h>
using namespace std;

char TEST[] = "2\n"
"hello\n"
"world\n"
"hi\n"
"world\n";

bool common_substring(string string1, string string2) {
  string trimmed1;
  string trimmed2;

  for (int i = 0; i < string1.length(); i++) {
    if (trimmed1.find(string1[i]) == string::npos) {
      trimmed1 += string1[i];
    }
  }

  for (int i = 0; i < string2.length(); i++) {
    if (trimmed2.find(string2[i]) == string::npos) {
      trimmed2 += string2[i];
    }
  }

  for (int i = 0; i < trimmed1.length(); i++) {
    for (int j = 0; j < trimmed2.length(); j++) {
      if (trimmed1[i] == trimmed2[j]) {
        return true;
      }
    }
  }

  return false;
}

void solve(istream &input) {
  int test_cases = 0;
  string string1, string2;

  input >> test_cases;

  for (; test_cases; test_cases--) {
    input >> string1 >> string2;

    if (common_substring(string1, string2)) {
      cout << "YES" << endl;
    } else {
      cout << "NO" << endl;
    }
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
