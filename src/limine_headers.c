#include <constants.h>
#include <debug.h>
#include <limine.h>
#include <limine_headers.h>
#include <types.h>

// Limine version
__attribute__((used, section(".limine_requests"))) static volatile uint64_t limine_base_revision[] =
    LIMINE_BASE_REVISION(6);

// Limine start/end header
__attribute__((
    used,
    section(".limine_requests_start"))) static volatile uint64_t limine_requests_start_marker[] =
    LIMINE_REQUESTS_START_MARKER;

__attribute__((
    used, section(".limine_requests_end"))) static volatile uint64_t limine_requests_end_marker[] =
    LIMINE_REQUESTS_END_MARKER;

// RSDP
__attribute__((
    used, section(".limine_requests"))) static volatile struct limine_rsdp_request rsdp_request = {
    .id = LIMINE_RSDP_REQUEST_ID, .revision = 0};

// HHDM
__attribute__((
    used, section(".limine_requests"))) static volatile struct limine_hhdm_request hhdm_request = {
    .id = LIMINE_HHDM_REQUEST_ID, .revision = 0};

/**
 * return RSDP pointer
 */
void* getRSDP() {
    struct limine_rsdp_response* response = rsdp_request.response;
    return response ? response->address : NULL;
}

uint64_t getHHDM() {
    struct limine_hhdm_response* response = hhdm_request.response;
    return response ? response->offset : 0;
}
