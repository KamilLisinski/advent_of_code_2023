#include <stdio.h>
#include <stdint.h>

struct v
{
    __uint64_t digit;
    __uint64_t mask;
    __uint64_t value;
};

const struct v digits[18] = {
    {'1',
     0xff,
     1},
    {'2',
     0xff,
     2},
    {'3',
     0xff,
     3},
    {'4',
     0xff,
     4},
    {'5',
     0xff,
     5},
    {'6',
     0xff,
     6},
    {'7',
     0xff,
     7},
    {'8',
     0xff,
     8},
    {'9',
     0xff,
     9},
    {'o' << 16 | 'n' << 8 | 'e',
     0xffffff,
     1},
    {'t' << 16 | 'w' << 8 | 'o',
     0xffffff,
     2},
    {116ul << 32 | 'h' << 24 | 'r' << 16 | 'e' << 8 | 'e',
     0xffffffffff,
     3},
    {'f' << 24 | 'o' << 16 | 'u' << 8 | 'r',
     0xffffffff,
     4},
    {'f' << 24 | 'i' << 16 | 'v' << 8 | 'e',
     0xffffffff,
     5},
    {'s' << 16 | 'i' << 8 | 'x',
     0xffffff,
     6},
    {115ul << 32 | 'e' << 24 | 'v' << 16 | 'e' << 8 | 'n',
     0xffffffffff,
     7},
    {101ul << 32 | 'i' << 24 | 'g' << 16 | 'h' << 8 | 't',
     0xffffffffff,
     8},
    {'n' << 24 | 'i' << 16 | 'n' << 8 | 'e',
     0xffffffff,
     9},
};

__uint64_t solve()
{
    __uint64_t current_sum = 0;
    __uint8_t first_in_line_found = 0;
    size_t i = 0;
    __uint64_t last_value = 0;
    __uint64_t current_value = 0;
    char c;
    while (1)
    {
        c = getchar();
        if (c == EOF || c == '\n')
        {
            current_sum += last_value;
            first_in_line_found = 0;
            last_value = 0;
            current_value = 0;
        }
        if (c == EOF)
        {
            break;
        }

        current_value <<= 8;
        current_value |= c;
        for (int j = 0; j < 18; j++)
        {
            // printf("c: 0x%016llX\n", current_value);
            // printf("m: 0x%016llX\n", digits[j].mask);
            // printf("d: 0x%016llX\n", digits[j].digit);
            // printf("v: 0x%016llX\n", current_value & digits[j].mask);

            if ((current_value & digits[j].mask) == digits[j].digit)
            {
                last_value = digits[j].value;
                if (first_in_line_found == 0)
                {
                    first_in_line_found = 1;
                    current_sum += last_value * 10;
                }
                break;
            }
        }
        i++;
    }
    return current_sum;
}

void main()
{
    printf("%lu\n", solve());
}