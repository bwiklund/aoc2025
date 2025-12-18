#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int count;
} Reveal;

int peekc(FILE *f) {
    int ch = fgetc(f);
    if (ch != EOF)
        ungetc(ch, f);
    return ch;
}

int parse_game(FILE *f) {
    int id;
    char line[256];
    if (!fgets(line, sizeof line, f)) {
        return -1;
    }

    sscanf(line, "Game %d: ", &id);
    printf("\nGame: %d\n", id);

    sscanf(line, "%d", &id);
    printf("\nGame: %d\n", id);

    return 0;

    // Reveal reveals[100];
    // while (true) {
    //     int count;
    //     char word[64];
    //     sscanf(f, "%d %63[a-z]", &count, word);
    //     printf("%d %s\n", count, word);

    //     int after = fgetc(f);
    //     if (after == '\n') {
    //         break;
    //     } else if (after == ';') {
    //         sscanf(f, " ");
    //         continue;
    //     } else if (after == ',') {
    //         sscanf(f, " ");
    //         continue;
    //     }
    // }
    // if (accept_str(f, "Game ") == -1) {
    //     return -1; // fail
    // }
    // int game_id;
    // if ((game_id = accept_number(f)) == -1) {
    //     return -1; // fail
    // }
    // printf("offset: %c\n", (char)fgetc(f));
    // printf("offset: %c\n", (char)fgetc(f));
    // if (accept_str(f, ": ") == -1) {
    //     return -1;
    // }
    // printf("offset: %d\n", game_id);
    // int n;
    // if ((n = accept_number(f)) == -1) {
    //     return -1;
    // }
    // printf("game id: %d %d\n", game_id, n);
    // if (accept_str(f, ": ") == -1) {
    //     return -1;
    // }
    // // while ((fgetc(f) != '\n'))
    // //     ;
    // return n;
    // return 0;
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