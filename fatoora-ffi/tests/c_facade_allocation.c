/* Inject C allocation failure; the Rust allocator remains unchanged. */
#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

static int fail_at = -1;
static int calls;
static int live;
static bool fail(void) { return calls++ == fail_at; }
static void *test_malloc(size_t size) {
    if (fail()) return NULL;
    void *p = malloc(size);
    if (p) ++live;
    return p;
}
static void *test_calloc(size_t count, size_t size) {
    if (fail()) return NULL;
    void *p = calloc(count, size);
    if (p) ++live;
    return p;
}
static void *test_realloc(void *p, size_t size) {
    if (fail()) return NULL;
    bool was_null = p == NULL;
    void *next = realloc(p, size);
    if (next && was_null) ++live;
    return next;
}
static void test_free(void *p) {
    if (p) --live;
    free(p);
}
#define malloc test_malloc
#define calloc test_calloc
#define realloc test_realloc
#define free test_free
#include "../../bindings/c/fatoora.c"
#undef malloc
#undef calloc
#undef realloc
#undef free

int main(void) {
    FatooraComplianceCsid *credentials = NULL;
    assert(fatoora_compliance_csid_create(FATOORA_SIMULATION, "", "a long token", "secret", &credentials, NULL) == FATOORA_OK);
    for (int allocation = 0; allocation < 9; ++allocation) {
        fail_at = allocation;
        calls = 0;
        char *token = (char *)credentials;
        FatooraError *error = NULL;
        FatooraStatus status = fatoora_compliance_csid_binary_security_token(credentials, &token, &error);
        if (allocation < 2) {
            assert(status == FATOORA_INTERNAL);
            assert(!token); /* Growth failure must never produce truncated success. */
        } else {
            assert(status == FATOORA_OK);
            assert(strcmp(token, "a long token") == 0);
        }
        fatoora_string_free(token);
        fatoora_error_free(error);
        assert(live == 0);
        calls = 0;
        token = NULL;
        error = NULL;
        status = fatoora_compliance_csid_request_id(credentials, &token, &error);
        if (allocation == 0) {
            assert(status == FATOORA_INTERNAL);
            assert(!token);
        } else {
            assert(status == FATOORA_OK);
            assert(token && !*token); /* Empty string still owns one byte. */
        }
        fatoora_string_free(token);
        fatoora_error_free(error);
        assert(live == 0);
        calls = 0;
        FatooraConfig *config = NULL;
        error = NULL;
        status = fatoora_config_new(255, &config, &error);
        assert(status == FATOORA_INVALID_INPUT); /* Preserve the native error code under OOM. */
        assert(!config);
        if (error) assert(fatoora_error_code(error) == status);
        fatoora_error_free(error);
        assert(live == 0);
    }
    fatoora_compliance_csid_free(credentials);
    puts("C API allocation contract passed");
    return 0;
}
