#ifndef DEBUG_H
#define DEBUG_H

#include <types.h>
#include <constants.h>

#define KPRINT_AUTO(v) _Generic((v), \
    char *:       kprint,            \
    const char *: kprint,            \
    default:      kprint_hex)(v)

#define DEBUG_PRINT_1(x)    do { kprint(x); kprint_new_line(); } while (0)
#define DEBUG_PRINT_2(x, y) do { kprint(x); KPRINT_AUTO(y); kprint_new_line(); } while (0)
#define GET_MACRO(_1, _2, NAME, ...) NAME

#if DEBUG == 1
#define DEBUG_PRINT(...) GET_MACRO(__VA_ARGS__, DEBUG_PRINT_2, DEBUG_PRINT_1)(__VA_ARGS__)
#else
#define DEBUG_PRINT(...) ((void)0)
#endif

void kprint(const char* s);
void kput_char(const char c);
void kprint_new_line();
void kprint_hex(uint64_t hex);
void kprint_int(int64_t number);

#endif
