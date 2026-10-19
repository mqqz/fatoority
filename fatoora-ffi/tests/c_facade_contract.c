#include "fatoora.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void failure(FatooraStatus status, FatooraError **error) {
    assert(status != FATOORA_OK);
    assert(*error);
    assert(fatoora_error_code(*error) == status);
    assert(strlen(fatoora_error_message(*error)));
    assert(strlen(fatoora_error_details_json(*error)));
    fatoora_error_free(*error);
    *error = NULL;
}

static FatooraInvoiceBuilder *builder(void) {
    FatooraInvoiceBuilder *b = NULL;
    FatooraInvoiceOptions options = {.kind = FATOORA_TAX, .subtype = FATOORA_SIMPLIFIED};
    assert(fatoora_invoice_builder_new(&options, &b, NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_id(b, "INV-1", NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_uuid(b, "8e6000cf-1a98-4174-b3e7-b5d5954bc10d", NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_issue_datetime(b, "2024-01-01T12:30:00Z", NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_currency(b, "SAR", NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_previous_invoice_hash(b, "hash", NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_invoice_counter(b, 1, NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_payment_means_code(b, "10", NULL) == FATOORA_OK);
    assert(fatoora_invoice_builder_set_vat_category(b, FATOORA_VAT_STANDARD, NULL) == FATOORA_OK);
    FatooraAddress *address = NULL;
    FatooraAddressOptions a = {
        .country_code = "SA", .city = "Riyadh", .street = "King Fahd",
        .building_number = "1234", .postal_code = "12222", .district = "Olaya"
    };
    assert(fatoora_address_new(&a, &address, NULL) == FATOORA_OK);
    FatooraPartyOptions seller = {.name = "Seller", .address = address, .vat_id = "399999999900003"};
    assert(fatoora_invoice_builder_set_seller(b, &seller, NULL) == FATOORA_OK);
    fatoora_address_free(address);
    return b;
}

static void invoices(const char *fixture) {
    FatooraError *error = NULL;
    FatooraInvoiceBuilder *b = builder();
    FatooraLineItemOptions item = {
        .description = "Item", .quantity = "3", .unit_code = "PCE",
        .unit_price = "invalid", .vat_rate = "15", .category = FATOORA_VAT_STANDARD
    };
    FatooraStatus status = fatoora_invoice_builder_add_line_item(b, &item, &error);
    assert(status == FATOORA_INVALID_INPUT);
    assert(strstr(fatoora_error_details_json(error), "invalid_decimal"));
    failure(status, &error);
    item.unit_price = "0.3333";
    assert(fatoora_invoice_builder_add_line_item(b, &item, &error) == FATOORA_OK);
    FatooraFinalizedInvoice *invoice = NULL;
    assert(fatoora_invoice_builder_build(b, &invoice, &error) == FATOORA_OK);
    FatooraInvoiceData *data = NULL;
    FatooraInvoiceTotals *totals = NULL;
    assert(fatoora_finalized_invoice_data(invoice, &data, &error) == FATOORA_OK);
    assert(fatoora_finalized_invoice_totals(invoice, &totals, &error) == FATOORA_OK);
    char *xml = NULL;
    assert(fatoora_finalized_invoice_xml(invoice, &xml, &error) == FATOORA_OK);
    fatoora_finalized_invoice_free(invoice);
    assert(strstr(xml, ">0.3333</cbc:PriceAmount>"));
    assert(strstr(xml, ">1.15</cbc:TaxInclusiveAmount>"));
    fatoora_string_free(xml);
    char *value = NULL;
    assert(fatoora_invoice_totals_tax_inclusive(totals, &value, &error) == FATOORA_OK);
    assert(strcmp(value, "1.15") == 0);
    fatoora_string_free(value);
    fatoora_invoice_totals_free(totals);
    assert(fatoora_invoice_data_id(data, &value, &error) == FATOORA_OK);
    assert(strcmp(value, "INV-1") == 0);
    fatoora_string_free(value);
    FatooraParty *buyer = NULL;
    assert(fatoora_invoice_data_buyer(data, &buyer, &error) == FATOORA_OK);
    assert(!buyer);
    FatooraInvoiceLineItem *line = NULL;
    failure(fatoora_invoice_data_line_item(data, 99, &line, &error), &error);
    assert(!line);
    fatoora_invoice_data_free(data);
    invoice = (FatooraFinalizedInvoice *)b; /* Sentinel must be cleared on failure. */
    failure(fatoora_invoice_builder_build(b, &invoice, &error), &error);
    assert(!invoice);
    failure(fatoora_invoice_builder_set_id(b, "again", &error), &error);
    fatoora_invoice_builder_free(b);

    /* Validation failure consumes the builder too. */
    FatooraInvoiceOptions options = {.subtype = FATOORA_SIMPLIFIED};
    assert(fatoora_invoice_builder_new(&options, &b, &error) == FATOORA_OK);
    failure(fatoora_invoice_builder_build(b, &invoice, &error), &error);
    failure(fatoora_invoice_builder_set_id(b, "again", &error), &error);
    fatoora_invoice_builder_free(b);

    FatooraSignedInvoice *signed_invoice = NULL;
    assert(fatoora_signed_invoice_from_file(fixture, &signed_invoice, &error) == FATOORA_OK);
    char *computed_hash = NULL, *raw_hash = NULL;
    assert(fatoora_signed_invoice_hash_base64(signed_invoice, &computed_hash, &error) == FATOORA_OK);
    assert(fatoora_signed_invoice_xml(signed_invoice, &xml, &error) == FATOORA_OK);
    assert(fatoora_xml_hash(xml, &raw_hash, &error) == FATOORA_OK);
    assert(strcmp(computed_hash, raw_hash) == 0);
    fatoora_string_free(computed_hash);
    fatoora_string_free(raw_hash);
    fatoora_string_free(xml);
    assert(fatoora_signed_invoice_into_xml(signed_invoice, &xml, &error) == FATOORA_OK);
    assert(strstr(xml, "Invoice"));
    failure(fatoora_signed_invoice_xml(signed_invoice, &value, &error), &error);
    assert(!value);
    fatoora_signed_invoice_free(signed_invoice);
    fatoora_string_free(xml);
}

static void crypto(void) {
    FatooraSigningKey *key = NULL, *copy = NULL;
    FatooraBytes *der = NULL;
    assert(fatoora_signing_key_generate(&key, NULL) == FATOORA_OK);
    assert(fatoora_signing_key_to_der(key, &der, NULL) == FATOORA_OK);
    FatooraBytesView view = {0};
    assert(fatoora_bytes_as_slice(der, &view, NULL) == FATOORA_OK);
    assert(view.len);
    assert(fatoora_signing_key_from_der(view.data, view.len, &copy, NULL) == FATOORA_OK);
    fatoora_bytes_free(der);
    char *pem = NULL;
    assert(fatoora_signing_key_to_pem(copy, &pem, NULL) == FATOORA_OK);
    assert(strstr(pem, "PRIVATE KEY"));
    fatoora_string_free(pem);
    fatoora_signing_key_free(copy);
    fatoora_signing_key_free(key);
}

static void optionals_and_arguments(void) {
    FatooraError *error = NULL;
    FatooraCsidCompliance *credentials = NULL;
    char *request = NULL;
    assert(fatoora_csid_compliance_create(FATOORA_SIMULATION, NULL, "token", "secret", &credentials, &error) == FATOORA_OK);
    assert(fatoora_csid_compliance_request_id(credentials, &request, &error) == FATOORA_OK);
    assert(!request);
    fatoora_csid_compliance_free(credentials);
    assert(fatoora_csid_compliance_create(FATOORA_SIMULATION, "", "token", "secret", &credentials, &error) == FATOORA_OK);
    assert(fatoora_csid_compliance_request_id(credentials, &request, &error) == FATOORA_OK);
    assert(request && !*request);
    fatoora_csid_compliance_free(credentials);
    fatoora_string_free(request);
    FatooraConfig *config = NULL;
    failure(fatoora_config_new(256, &config, &error), &error);
    failure(fatoora_config_new(-1, &config, &error), &error);
    assert(fatoora_config_new(FATOORA_SIMULATION, &config, &error) == FATOORA_OK);
    failure(fatoora_xml_validate(config, NULL, NULL, &error), &error);
    fatoora_config_free(config);
    failure(fatoora_invoice_builder_set_id(NULL, "INV-1", &error), &error);
    FatooraInvoiceBuilder *b = NULL;
    failure(fatoora_invoice_builder_new(NULL, &b, &error), &error);
    FatooraInvoiceOptions options = {.subtype = FATOORA_SIMPLIFIED};
    failure(fatoora_invoice_builder_new(&options, NULL, &error), &error);
    assert(fatoora_invoice_builder_new(&options, &b, &error) == FATOORA_OK);
    const char bad_utf8[] = {(char)0xff, 0};
    failure(fatoora_invoice_builder_set_id(b, bad_utf8, &error), &error);
    fatoora_invoice_builder_free(b);
    FatooraSigningKey *key = NULL;
    failure(fatoora_signing_key_from_der(NULL, 1, &key, &error), &error);
    failure(fatoora_signing_key_from_der((const uint8_t *)"", SIZE_MAX, &key, &error), &error);
    failure(fatoora_signing_key_from_der(NULL, 0, &key, &error), &error);
    fatoora_string_free(NULL);
    fatoora_error_free(NULL);
    fatoora_config_free(NULL);
}

int main(int argc, char **argv) {
    assert(argc == 2);
    invoices(argv[1]);
    crypto();
    optionals_and_arguments();
    puts("C API contract passed");
    return 0;
}
