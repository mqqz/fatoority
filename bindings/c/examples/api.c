/* --8<-- [start:example] */
#include "fatoora.h"
#include <stdio.h>

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(argc == 1 ? stdout : stderr,
            "Usage: %s signed-invoice.xml token secret\n"
            "Submits a simplified invoice to the non-production gateway.\n", argv[0]);
        return argc == 1 ? 0 : 1;
    }
    FatooraConfig *config = NULL;
    FatooraSignedInvoice *invoice = NULL;
    FatooraProductionCsid *credentials = NULL;
    FatooraZatcaClient *client = NULL;
    FatooraValidationResponse *response = NULL;
    FatooraError *error = NULL;
    FatooraStatus status = FATOORA_OK;
    int exit_code = 1;
#define TRY(call) do { status = (call); if (status != FATOORA_OK) goto cleanup; } while (0)
    TRY(fatoora_signed_invoice_from_file(argv[1], &invoice, &error));
    TRY(fatoora_config_new(FATOORA_NON_PRODUCTION, &config, &error));
    TRY(fatoora_zatca_client_new(config, &client, &error));
    TRY(fatoora_production_csid_new(FATOORA_NON_PRODUCTION, NULL,
        argv[2], argv[3], &credentials, &error));
    TRY(fatoora_zatca_client_report_simplified_invoice(client, invoice,
        credentials, FATOORA_CLEARANCE_DISABLED, "en", &response, &error));
    /* Receiving a response does not establish acceptance. */
    TRY(fatoora_validation_response_ensure_accepted(response, &error));
    puts("Invoice reporting accepted.");
    exit_code = 0;
cleanup:
    if (exit_code) fprintf(stderr, "Error %d: %s\n%s\n", (int)status,
        fatoora_error_message(error), fatoora_error_details_json(error));
    fatoora_validation_response_free(response);
    fatoora_zatca_client_free(client);
    fatoora_production_csid_free(credentials);
    fatoora_signed_invoice_free(invoice);
    fatoora_config_free(config);
    fatoora_error_free(error);
    return exit_code;
}
/* --8<-- [end:example] */
