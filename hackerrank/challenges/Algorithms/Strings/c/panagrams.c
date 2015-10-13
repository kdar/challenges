#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <ctype.h>
#include <assert.h>

char *TEST = "We promptly judged antique ivory buckles for the next prize";

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
  char checked['z'-'a'+1] = {0};
  int count = 0;
  char line[10*10*10] = {0};

  fscanf(input, "%[^\n]", (char*)&line);

  for (int i = 0; line[i]; i++) {
    char c = tolower(line[i]);
    if (c >= 'a' && c <= 'z') {
      int index = tolower(line[i])-'a';

      if (checked[index] == 0) {
        checked[index] = 1;
        count++;
        if (count == 'z'-'a'+1) {
          printf("pangram");
          return;
        }
      }
    }
  }

  printf("not pangram");
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
