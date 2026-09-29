#include "arch/x64/serial.h"
#include "arch/x64/io.h"

void serial_put_char(const char c) {
    outb(COM1, c);
}

void serial_print(const char* s) {
    while (*s) {
        serial_put_char(*s);
        s++;
    }
}
