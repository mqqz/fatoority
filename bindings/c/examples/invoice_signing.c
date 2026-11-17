/* --8<-- [start:example] */
#include "fatoora.h"
#include <stdio.h>
#include <stdlib.h>

/* File I/O belongs to the application; the signing API takes byte buffers. */
static unsigned char *read_der(const char *path, size_t *length) {
    FILE *file = fopen(path, "rb");
    if (!file) return NULL;
    unsigned char *data = NULL;
    if (fseek(file, 0, SEEK_END)) goto done;
    long size = ftell(file);
    if (size <= 0 || size > 1024 * 1024) goto done;
    rewind(file);
    data = malloc((size_t)size);
    if (!data) goto done;
    if (fread(data, 1, (size_t)size, file) != (size_t)size) {
        free(data);
        data = NULL;
    } else {
        *length = (size_t)size;
    }
done:
    fclose(file);
    return data;
}

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "Usage: %s invoice.xml certificate.der private-key.der\n", argv[0]);
        return 1;
    }
    size_t cert_len = 0, key_len = 0;
    unsigned char *cert = read_der(argv[2], &cert_len);
    unsigned char *key = read_der(argv[3], &key_len);
    if (!cert || !key) {
        fputs("Unable to read certificate or private key DER.\n", stderr);
        free(cert);
        free(key);
        return 1;
    }
    FatooraInvoiceSigner *signer = NULL;
    FatooraFinalizedInvoice *invoice = NULL;
    FatooraSignedInvoice *signed_invoice = NULL;
    FatooraError *error = NULL;
    char *xml = NULL;
    FatooraStatus status = FATOORA_OK;
    int exit_code = 1;
#define TRY(call) do { status = (call); if (status != FATOORA_OK) goto cleanup; } while (0)
    TRY(fatoora_invoice_signer_from_der(cert, cert_len, key, key_len, &signer, &error));
    TRY(fatoora_finalized_invoice_from_file(argv[1], &invoice, &error));
    /* Signing consumes invoice's inner state; its outer handle still needs free. */
    TRY(fatoora_invoice_signer_sign(signer, invoice, &signed_invoice, &error));
    TRY(fatoora_signed_invoice_into_xml(signed_invoice, &xml, &error));
    puts(xml);
    exit_code = 0;
cleanup:
    if (exit_code) fprintf(stderr, "Error %d: %s\n", (int)status, fatoora_error_message(error));
    fatoora_string_free(xml);
    fatoora_signed_invoice_free(signed_invoice);
    fatoora_finalized_invoice_free(invoice);
    fatoora_invoice_signer_free(signer);
    fatoora_error_free(error);
    free(cert);
    free(key);
    return exit_code;
}
/* --8<-- [end:example] */
