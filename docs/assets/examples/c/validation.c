#include "fatoora.h"
#include <stdio.h>
#include <stdlib.h>

static char *read_xml(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) return NULL;
    char *xml = NULL;
    if (fseek(file, 0, SEEK_END)) goto done;
    long size = ftell(file);
    if (size < 0 || size > 1024 * 1024) goto done;
    rewind(file);
    xml = malloc((size_t)size + 1);
    if (!xml) goto done;
    if (fread(xml, 1, (size_t)size, file) != (size_t)size) {
        free(xml);
        xml = NULL;
    } else {
        xml[size] = '\0';
    }
done:
    fclose(file);
    return xml;
}

int main(int argc, char **argv) {
    if (argc != 2 && argc != 3) {
        fprintf(stderr, "Usage: %s invoice.xml [validation-options.json]\n", argv[0]);
        return 1;
    }
    FatooraError *error = NULL;
    char *xml = read_xml(argv[1]);
    if (!xml) {
        fputs("Unable to read invoice XML.\n", stderr);
        return 1;
    }
    bool valid = false, complete = false;
    char *options = NULL, *json = NULL;
    FatooraZatcaValidationReport *report = NULL;
    FatooraStatus status = FATOORA_OK;
#define TRY(call) do { status = (call); if (status != FATOORA_OK) goto cleanup; } while (0)
    /* Validate original XML; parsing/reserializing would change the input. */
    if (argc == 3) {
        options = read_xml(argv[2]);
        if (!options) { fputs("Unable to read validation options.\n", stderr); goto cleanup; }
        TRY(fatoora_xml_validate_zatca(xml, options, &report, &error));
        TRY(fatoora_zatca_validation_report_is_valid(report, &valid, &error));
        TRY(fatoora_zatca_validation_report_is_complete(report, &complete, &error));
        TRY(fatoora_zatca_validation_report_to_json(report, &json, &error));
        puts(json);
        printf("valid: %s, complete: %s\n", valid ? "true" : "false", complete ? "true" : "false");
        /* Exit 0 means the report was produced, not that the invoice passed. */
        valid = true;
    } else {
        TRY(fatoora_xml_validate(xml, &valid, &error));
        printf("XSD valid: %s\n", valid ? "true" : "false");
    }
cleanup:
    if (status != FATOORA_OK)
        fprintf(stderr, "Error %d: %s\n", (int)status, fatoora_error_message(error));
    free(options);
    fatoora_string_free(json);
    fatoora_zatca_validation_report_free(report);
    free(xml);
    fatoora_error_free(error);
    return status == FATOORA_OK && valid ? 0 : 1;
}
