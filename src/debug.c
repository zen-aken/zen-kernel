#include <debug.h>
#include <string.h>

#include "arch/x64/serial.h"

char* hex_chars = "0123456789ABCDEF";

void kprint(char* s) {
    serial_print(s);
}

void kprint_new_line(){
    serial_print("\n");
}

void kprint_int(int64_t number) {
    char buffer[22];
    serial_print(int_to_str(buffer, number));
}

void kprint_hex(uint64_t hex) {
    char buffer[18];
    kprint(hex_to_str(buffer, hex));
}
