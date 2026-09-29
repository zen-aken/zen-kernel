#ifndef STRING_H
#define STRING_H

#include <types.h>

#define MAX_HEX_BUFFER_SIZE 19
#define MAX_INT_BUFFER_SIZE 22

char* hex_to_str(char buffer[MAX_HEX_BUFFER_SIZE], uint64_t hex);
char* int_to_str(char buffer[MAX_INT_BUFFER_SIZE], int64_t number);

#endif
