#include "fatoora.h"
#include <stdio.h>

/* All amounts are decimal strings, as in the Rust and Python bindings. */
int main(void) {
    FatooraError *error = NULL;
    FatooraInvoiceBuilder *builder = NULL;
    FatooraPaymentMeans *means = NULL;
    FatooraAddress *address = NULL;
    FatooraFinalizedInvoice *invoice = NULL;
    char *xml = NULL;
    int exit_code = 1;
    FatooraStatus status = FATOORA_OK;

#define CHECK(call) do { status = (call); if (status != FATOORA_OK) goto cleanup; } while (0)
    FatooraInvoiceOptions options = {
        .kind = FATOORA_TAX, .subtype = FATOORA_SIMPLIFIED
    };
    CHECK(fatoora_invoice_builder_new(&options, &builder, &error));
    CHECK(fatoora_invoice_builder_set_id(builder, "INV-1", &error));
    CHECK(fatoora_invoice_builder_set_uuid(builder,
        "8e6000cf-1a98-4174-b3e7-b5d5954bc10d", &error));
    CHECK(fatoora_invoice_builder_set_issue_datetime(builder,
        "2024-01-01T12:30:00Z", &error));
    CHECK(fatoora_invoice_builder_set_currency(builder, "SAR", &error));
    CHECK(fatoora_invoice_builder_set_previous_invoice_hash(builder, "hash", &error));
    CHECK(fatoora_invoice_builder_set_invoice_counter(builder, 1, &error));
    CHECK(fatoora_payment_means_new("10", NULL, &means, &error));
    CHECK(fatoora_invoice_builder_add_payment_means(builder, means, &error));
    CHECK(fatoora_invoice_builder_set_vat_category(builder, FATOORA_VAT_STANDARD, &error));

    FatooraAddressOptions address_options = {
        .country_code = "SA", .city = "Riyadh", .street = "King Fahd",
        .building_number = "1234", .postal_code = "12222", .district = "Olaya"
    };
    CHECK(fatoora_address_new(&address_options, &address, &error));
    FatooraPartyOptions seller = {
        .name = "Seller", .address = address, .vat_id = "399999999900003",
        .other_id = "7003339333", .scheme = "CRN"
    };
    CHECK(fatoora_invoice_builder_set_seller(builder, &seller, &error));
    FatooraLineItemOptions item = {
        .description = "Item", .quantity = "3", .unit_code = "PCE",
        .unit_price = "0.3333", .vat_rate = "15", .category = FATOORA_VAT_STANDARD
    };
    CHECK(fatoora_invoice_builder_add_line_item(builder, &item, &error));
    CHECK(fatoora_invoice_builder_build(builder, &invoice, &error));
    CHECK(fatoora_finalized_invoice_xml(invoice, &xml, &error));
    puts(xml);
    exit_code = 0;

cleanup:
    if (exit_code) fprintf(stderr, "Error %d: %s\n",
        (int)status, fatoora_error_message(error));
    fatoora_string_free(xml);
    fatoora_finalized_invoice_free(invoice);
    fatoora_address_free(address);
    fatoora_payment_means_free(means);
    fatoora_invoice_builder_free(builder);
    fatoora_error_free(error);
    return exit_code;
}
