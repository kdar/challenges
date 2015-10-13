#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <iomanip>
using namespace std;


int main() {
  double L, S1, S2;
  int Q;

  cin >> L >> S1 >> S2;
  cin >> Q;

  while (Q-- > 0) {
    double qi;
    cin >> qi;

    // time = distance/velocity (t = d/v)
    // diagonal of square = sqrt(2)*side (di = sqrt(2)*a)
    // length of side given area of square = sqrt(area)
    // So we need to get the difference in distance of the diagonal of
    // the overlapping square, and the square:
    // d = L*sqrt(2) - sqrt(qi)*sqrt(2)
    // We need the relative speed:
    // v = abs(S1-S2)
    // therefore, given these two equations and t = d/v
    // t = (L*sqrt(2) - sqrt(qi)*sqrt(2))/abs(S1-S2) = ((L-sqrt(qi))*sqrt(2))/abs(S1-S2)
    cout << setprecision(20) << ((L-sqrt(qi))*sqrt(2))/abs(S1-S2) << endl;
  }

  return 0;
}
