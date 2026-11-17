/* --8<-- [start:example] */
#include "SignedInvoice.h"
#include "BindingError.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>
#ifndef FATOORA_DOC_SIGNED_XML
#define FATOORA_DOC_SIGNED_XML "path/to/signed_invoice.xml"
#endif
static DiplomatStringView s(const char *v) { return (DiplomatStringView){v,strlen(v)}; }
#include "Xml.h"
int main(void) {
    fatoora_SignedInvoice_from_file_result invoice = fatoora_SignedInvoice_from_file(s(FATOORA_DOC_SIGNED_XML));
    assert(invoice.is_ok);
    DiplomatWrite *xml = diplomat_buffer_write_create(0);
    fatoora_SignedInvoice_xml_result copied = fatoora_SignedInvoice_xml(invoice.ok, xml);
    assert(copied.is_ok);
    fatoora_SignedInvoice_destroy(invoice.ok);
    DiplomatStringView view = {(const char *)diplomat_buffer_write_get_bytes(xml), diplomat_buffer_write_len(xml)};
    fatoora_Xml_validate_result valid = fatoora_Xml_validate(view);
    assert(valid.is_ok && valid.ok);
    diplomat_buffer_write_destroy(xml);
    return 0;
}
/* --8<-- [end:example] */
