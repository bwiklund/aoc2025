#include <stdio.h>
#include <stdbool.h>

int main()
{
    char ch;
    FILE *f = fopen("day1_input.txt", "r");

    int first_num = -1; // how to avoid magic numbers like this in c. or is this just how we roll
    int last_num = -1;
    int sum = 0;

    while ((ch = fgetc(f)) != EOF)
    {
        bool is_number = ch >= 48 && ch <= 57;
        if (is_number)
        {
            int number = ch - 48;
            if (first_num == -1)
            {
                first_num = number;
            }
            last_num = number;
        }
        if (ch == '\n')
        {
            // printf("%d, %d\n", first_num, last_num);
            int calibration_number = first_num * 10 + last_num;
            sum += calibration_number;

            first_num = -1;
            last_num = -1;
        }
    }
    fclose(f);

    printf("%d\n", sum);

    return 0;
}