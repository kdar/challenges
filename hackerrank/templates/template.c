#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <ctype.h>
#include <assert.h>

char *TEST = "";

#ifdef _WIN32
FILE *fmemopen(void *buf, size_t size, const char *opentype) {
  FILE *f;

  assert(strcmp(opentype, "r") == 0);

  f = tmpfile();
  fwrite(buf, 1, size, f);
  rewind(f);

  return f;
}
#endif

void solve(FILE *input) {
  // char line[100] = {0};
  // fscanf(input, "%[^\n]", (char*)&line);
}

int main(int argc, char *argv[]) {
  // for testing purposes
  if (argc == 2 && strcmp(argv[1], "test") == 0) {
    FILE *stream = fmemopen(TEST, strlen(TEST), "r");
    solve(stream);
    fclose(stream);
  } else {
    solve(stdin);
  }
  return 0;
}
