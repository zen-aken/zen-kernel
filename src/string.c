#include <string.h>

/**
 * @brief Turns a uint64_t into a hexadecimal string.
 * @param buffer Output buffer, minimum 17 bytes.
 * @param hex The value to convert.
 * @return buffer
 */
char* hex_to_str(char buffer[19], uint64_t hex) {
    const char hex_table[] = "0123456789ABCDEF";

    buffer[0] = '0';
    buffer[1] = 'x';

    for (uint8_t i = 0; i < 16; i++) {
        buffer[i + 2] = hex_table[(hex >> ((15 - i) * 4)) & 0xF];
    }

    buffer[19] = '\0';

    return buffer;
}

/**
    @brief Converts a signed 64-bit integer to a null-terminated string.
    The resulting string is written to the provided buffer in decimal
    notation, including a leading '-' for negative values.
    @param buffer Output buffer. Must be at least 22 bytes.
    @param number The signed 64-bit integer to convert.
    @return The provided buffer containing the resulting string.
    */
char* int_to_str(char buffer[22], int64_t number) {
    const char numbers[] = "0123456789";

    if (number == 0) {
        buffer[0] = '0';
        buffer[1] = '\0';
        return buffer;
    }

    bool isNegative = number < 0;

    uint64_t magnitude = isNegative ? (uint64_t)(-(number + 1)) + 1 : (uint64_t)number;

    uint8_t length = 0;

    while (magnitude != 0) {
        buffer[length++] = numbers[magnitude % 10];
        magnitude /= 10;
    }

    for (uint8_t i = 0; i < length / 2; i++) {
        char temp = buffer[i];
        buffer[i] = buffer[length - 1 - i];
        buffer[length - 1 - i] = temp;
    }

    if (isNegative) {
        for (uint8_t i = length; i > 0; i--) {
            buffer[i] = buffer[i - 1];
        }

        buffer[0] = '-';
        length++;
    }

    buffer[length] = '\0';

    return buffer;
}
