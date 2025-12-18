#include <stdio.h>

int main() {
  char ch;
  FILE *f = fopen("day1_input.txt", "r");

  while ((ch = fgetc(f)) != EOF) printf("%c", ch);

  fclose(f);
  return 0;
}