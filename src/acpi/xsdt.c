#include <acpi/xsdt.h>
#include <constants.h>
#include <debug.h>
#include <limine_headers.h>
#include <string.h>
#include <types.h>

/**
 * @brief Validate the RSDP and return the XSDT address.
 *
 * Checks the "RSD PTR " signature, the 20-byte checksum and, for
 * revision >= 2, the extended checksum. Only ACPI 2.0+ is supported;
 * on revision 0 (RSDT only) the function fails.
 *
 * @return Virtual address of the XSDT, or NULL if the RSDP
 *         is missing, invalid, or has no XSDT.
 */
void* getXSDT() {
    struct RSDP_t* rsdp = getRSDP();

    if (!rsdp) {
        DEBUG_PRINT("RSPD is NULL");
        return NULL;
    }

    if (memcmp(rsdp->Signature, "RSD PTR ", 8)) {
        DEBUG_PRINT("Signature mismatch");
        return NULL;
    }

    char sig[9];
    memcpy(sig, rsdp->Signature, 8);
    sig[8] = '\0';

    DEBUG_PRINT("RSDP signature: ", sig);
    DEBUG_PRINT("RSDP address: ", (uint64_t)rsdp);
    DEBUG_PRINT("RSDP revision: ", rsdp->Revision);

    uint8_t sum = 0;
    for (size_t i = 0; i < 20; i++) {
        sum += ((uint8_t*)rsdp)[i];
    }
    if (sum != 0) {
        DEBUG_PRINT("Invalid checksum for RSDP");
        return NULL;
    }

    if (rsdp->Revision >= 2) {
        struct XSDP_t* xsdp = (struct XSDP_t*)rsdp;

        uint8_t xsum = 0;
        for (uint32_t i = 0; i < xsdp->Length; i++) {
            xsum += ((uint8_t*)xsdp)[i];
        }
        if (xsum != 0) {
            DEBUG_PRINT("Invalid checksum for XSDP");
            return NULL;
        }

        uint64_t hhdm = getHHDM();

        DEBUG_PRINT("XSDP available switching...");
        DEBUG_PRINT("XSDP address: ", (uint64_t)xsdp);
        DEBUG_PRINT("XSDT address: ", xsdp->XsdtAddress + hhdm);

        return ((void*)xsdp->XsdtAddress + hhdm);
    }

    return NULL;
}

void* getTable(const char table_signature[4]) {
    struct XSDT_t* xsdt = (struct XSDT_t*)getXSDT();
    if (!xsdt) {
        DEBUG_PRINT("XSDT is NULL");
        return 0;
    }

    uint64_t entries = (xsdt->header.Length - sizeof(struct ACPI_SDT_Header)) / 8;

    DEBUG_PRINT("XSDT entry count: ", entries);

    uint64_t hhdm = getHHDM();
    DEBUG_PRINT("Available XSDT tables: ");
    for (uint64_t i = 0; i < entries; i++) {
        uint64_t phys = xsdt->PointerToOtherSDT[i];
        struct ACPI_SDT_Header* virt = (struct ACPI_SDT_Header*)(phys + hhdm);

        char buf[5];
        memcpy(buf, virt->Signature, 4);
        buf[4] = '\0';

        DEBUG_PRINT("-> ", buf);

        if (memcmp(table_signature, virt->Signature, 4) == 0) {
            return virt;
        }
    }

    return NULL;
}
