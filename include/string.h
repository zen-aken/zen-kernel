#ifndef STRING_H
#define STRING_H

#define MAX_HEX_BUFFER_SIZE 19
#define MAX_INT_BUFFER_SIZE 22

#include <types.h>

char* hex_to_str(char buffer[MAX_HEX_BUFFER_SIZE], uint64_t hex);
char* int_to_str(char buffer[MAX_INT_BUFFER_SIZE], int64_t number);
int memcmp(const void* a, const void* b, size_t n);
void* memcpy(void* dst, const void* src, size_t n);
void* memset(void* dst, int c, size_t n);
void* memmove(void* dst, const void* src, size_t n);
size_t strlen(const char* s);

#endif
