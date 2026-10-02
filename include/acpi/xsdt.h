#ifndef XSDT_H
#define XSDT_H

#include <types.h>

// structure for revision 0 (version 1.0)
struct RSDP_t {
    char Signature[8];
    uint8_t Checksum;
    char OEMID[6];
    uint8_t Revision;
    uint32_t RsdtAddress;
} __attribute__((packed));

// structure for revision 2 (version 2.0+)
struct XSDP_t {
    struct RSDP_t rsdp;
    uint32_t Length;
    uint64_t XsdtAddress;
    uint8_t ExtendedChecksum;
    uint8_t reserved[3];
} __attribute__((packed));

/**
 * @brief Common header at the start of every ACPI System Description Table.
 *
 * Total size is 36 bytes. Table-specific data follows the header.
 */
struct ACPI_SDT_Header {
    char Signature[4];
    uint32_t Length;
    uint8_t Revision;
    uint8_t Checksum;
    char OEMID[6];
    char OEMTableID[8];
    uint32_t OEMRevision;
    uint32_t CreatorID;
    uint32_t CreatorRevision;
};
_Static_assert(sizeof(struct ACPI_SDT_Header) == 36, "SDT header size");

/**
 * @brief XSDT (Extended System Description Table).
 *
 * Standard SDT header followed by an array of 64-bit physical addresses,
 * each pointing to another SDT. Entry count is
 * `(header.Length - sizeof(struct ACPI_SDT_Header)) / 8`.
 */
struct XSDT_t {
    struct ACPI_SDT_Header header;
    uint64_t PointerToOtherSDT[];
} __attribute__((packed));

void* getXSDT();
void* getTable(const char table_signature[4]);

#endif
