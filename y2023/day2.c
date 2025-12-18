#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int accept_char(FILE *f, char ch) {
    if (fgetc(f) == ch) {
        return true;
    } else {
        ungetc(1, f);
        return false;
    }
}

int accept_str(FILE *f, const char *str) {
    int len = strlen(str);
    for (int i = 0; i < len; i++) {
        if (!accept_char(f, str[i])) {
            ungetc(i, f);
            return false;
        }
    }
    return true;
}

int accept_digit(FILE *f) {
    char ch = fgetc(f);
    if ((ch >= '0' && ch <= '9')) {
        return ch - '0';
    } else {
        ungetc(1, f);
        return -1; // fail
    }
}

int accept_number(FILE *f) {
    int acc = 0;
    int d;
    int count = 0;
    while ((d = accept_digit(f)) != -1) {
        count += 1;
        acc *= 10;
        acc += d;
    }
    if (count == 0) {
        return -1; // fail
    }
    return acc;
}

int parse_game(FILE *f) {
    if (accept_str(f, "Game ") == -1) {
        return -1; // fail
    }
    int game_id;
    if ((game_id = accept_number(f)) == -1) {
        return -1; // fail
    }
    printf("offset: %c\n", (char)fgetc(f));
    printf("offset: %c\n", (char)fgetc(f));
    if (accept_str(f, ": ") == -1) {
        return -1;
    }
    printf("offset: %d\n", game_id);
    int n;
    if ((n = accept_number(f)) == -1) {
        return -1;
    }
    printf("game id: %d %d\n", game_id, n);
    if (accept_str(f, ": ") == -1) {
        return -1;
    }
    // while ((fgetc(f) != '\n'))
    //     ;
    return n;
}

int parse_input(FILE *f) {
    while (-1 != parse_game(f)) {
    }
    return 0;
}

int main() {
    FILE *f = fopen("day2_input.txt", "r");
    parse_input(f);
    return 0;
}