/* Compile as C and C++; argv[1] is the standard invoice SDK fixture. */
#include "Xml.h"
#include "ZatcaFinding.h"
#include "ZatcaStageReport.h"
#include "ZatcaValidationReport.h"
#include "BindingError.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char *read_invoice(const char *path) {
    FILE *file = fopen(path, "rb");
    assert(file != NULL);
    assert(fseek(file, 0, SEEK_END) == 0);
    long size = ftell(file);
    assert(size > 0 && size < 1000000);
    rewind(file);
    char *xml = (char *)malloc((size_t)size + 1);
    assert(xml != NULL);
    assert(fread(xml, 1, (size_t)size, file) == (size_t)size);
    xml[size] = '\0';
    fclose(file);
    return xml;
}

static DiplomatStringView view(const char *text) {
    DiplomatStringView value = {text, strlen(text)};
    return value;
}

static ZatcaValidationReport *validate(const char *xml, const char *options) {
    OptionStringView opts = {0};
    if (options != NULL) { opts.is_ok = true; opts.ok = view(options); }
    fatoora_Xml_validate_zatca_result result = fatoora_Xml_validate_zatca(view(xml), opts);
    assert(result.is_ok);
    return result.ok;
}

/* The JSON form keeps the pre-0.6 string output available. */
static DiplomatWrite *report(const char *xml, const char *options) {
    ZatcaValidationReport *value = validate(xml, options);
    DiplomatWrite *out = diplomat_buffer_write_create(0);
    assert(fatoora_ZatcaValidationReport_to_json(value, out).is_ok);
    fatoora_ZatcaValidationReport_destroy(value);
    return out;
}

static void invalid(DiplomatStringView xml, const char *options) {
    OptionStringView opts = {0};
    if (options != NULL) { opts.is_ok = true; opts.ok = view(options); }
    fatoora_Xml_validate_zatca_result result = fatoora_Xml_validate_zatca(xml, opts);
    assert(!result.is_ok && fatoora_BindingError_code(result.err) == 1);
    fatoora_BindingError_destroy(result.err);
}

int main(int argc, char **argv) {
    assert(argc == 2);
    char *xml = read_invoice(argv[1]);
    const char *options = "{\"previous_invoice_hash\":\"NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==\",\"evaluated_at\":\"2026-09-23T12:00:00+03:00\"}";
    DiplomatWrite *good = report(xml, options);
    assert(strstr(diplomat_buffer_write_get_bytes(good), "\"is_valid\":true"));
    assert(strstr(diplomat_buffer_write_get_bytes(good), "\"severity\":\"warning\""));
    ZatcaValidationReport *typed = validate(xml, options);
    assert(fatoora_ZatcaValidationReport_is_valid(typed));
    assert(fatoora_ZatcaValidationReport_is_complete(typed));
    assert(fatoora_ZatcaValidationReport_stages_len(typed) == 6);
    fatoora_ZatcaValidationReport_stage_result stage = fatoora_ZatcaValidationReport_stage(typed, 0);
    assert(stage.is_ok);
    fatoora_ZatcaValidationReport_destroy(typed);
    assert(fatoora_ZatcaStageReport_stage(stage.ok) == ZatcaStage_Xsd);
    assert(fatoora_ZatcaStageReport_status(stage.ok) == ZatcaStageStatus_Completed);
    fatoora_ZatcaStageReport_destroy(stage.ok);
    DiplomatWrite *missing = report(xml, NULL);
    assert(strstr(diplomat_buffer_write_get_bytes(missing), "context_required"));
    assert(strstr(diplomat_buffer_write_get_bytes(missing), "\"is_valid\":false"));
    diplomat_buffer_write_destroy(missing);
    DiplomatWrite *rejected = report("<wrong/>", options);
    assert(strstr(diplomat_buffer_write_get_bytes(rejected), "XSD_INVALID"));
    diplomat_buffer_write_destroy(rejected);
    OptionStringView defaults = {0};
    fatoora_Xml_validate_zatca_result bad = fatoora_Xml_validate_zatca(view("<Invoice"), defaults);
    assert(!bad.is_ok && fatoora_BindingError_code(bad.err) == 4);
    DiplomatWrite *details = diplomat_buffer_write_create(0);
    fatoora_BindingError_details_json(bad.err, details);
    fatoora_BindingError_destroy(bad.err);
    assert(strstr(diplomat_buffer_write_get_bytes(details), "zatca_validation_execution"));
    assert(strstr(diplomat_buffer_write_get_bytes(details), "\"report\":"));
    invalid(view(xml), "{\"unknown\":true}");
    invalid(view("\xff"), NULL);
    DiplomatStringView nul = {"<x/>\0ignored", 12};
    invalid(nul, NULL);
    char oversized_options[4098];
    memset(oversized_options, ' ', sizeof(oversized_options));
    oversized_options[4097] = '\0';
    invalid(view(xml), oversized_options);
    free(xml);
    assert(strstr(diplomat_buffer_write_get_bytes(good), "\"is_complete\":true"));
    assert(strstr(diplomat_buffer_write_get_bytes(details), "invalid_xml"));
    diplomat_buffer_write_destroy(good);
    diplomat_buffer_write_destroy(details);
    puts("ZATCA C report ownership and execution contracts passed");
    return 0;
}
