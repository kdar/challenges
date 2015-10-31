#include <iostream>

using namespace std;
int main()
{
    long int M, N, R;
    int i, j, k, l;
    cin >> M;
    cin >> N;
    cin >> R;

    long int a[M][N], b[M][N];
    for (i = 1; i <= M; i++)
        for (j = 1; j <= N; j++)
            cin >> a[i][j];


    if (M % 2 == 0) {
        k = M / 2;
    }
    else {
        k = M / 2 + 1;
    }
    if (N % 2 == 0) {
        l = N / 2;
    }
    else {
        l = N / 2 + 1;
    }
    while (R > 0) {
        for (i = 1; i <= k; i++)
            for (j = i; j <= N - i; j++) {
                b[i][j] = a[i][j + 1];
                b[M + 1 - i][N + 1 - j] = a[M + 1 - i][N - j];
            }
        for (j = 1; j <= l; j++)
            for (i = M + 1 - j; i >= j + 1; i--) {
                b[i][j] = a[i - 1][j];
                b[M + 1 - i][N + 1 - j] = a[M - i + 2][N + 1 - j];
            }

        for (i = 1; i <= M; i++)
            for (j = 1; j <= N; j++)
                a[i][j] = b[i][j];
        R--;
    }

    cout << endl;
    for (i = 1; i <= M; i++) {
        for (j = 1; j <= N; j++) {
            cout << a[i][j];
            cout << " ";
        }
        cout << endl;
    }
    return 0;
}
