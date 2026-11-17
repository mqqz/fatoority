#ifndef InvoiceSigner_H
#define InvoiceSigner_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Bytes.d.h"
#include "FinalizedInvoice.d.h"
#include "SignedInvoice.d.h"

#include "InvoiceSigner.d.h"






typedef struct fatoora_InvoiceSigner_from_pem_result {union {InvoiceSigner* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_from_pem_result;
fatoora_InvoiceSigner_from_pem_result fatoora_InvoiceSigner_from_pem(DiplomatStringView cert_pem, DiplomatStringView key_pem);

typedef struct fatoora_InvoiceSigner_from_der_result {union {InvoiceSigner* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_from_der_result;
fatoora_InvoiceSigner_from_der_result fatoora_InvoiceSigner_from_der(DiplomatU8View cert_der, DiplomatU8View key_der);

typedef struct fatoora_InvoiceSigner_certificate_der_result {union {Bytes* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_certificate_der_result;
fatoora_InvoiceSigner_certificate_der_result fatoora_InvoiceSigner_certificate_der(const InvoiceSigner* self);

typedef struct fatoora_InvoiceSigner_certificate_pem_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_certificate_pem_result;
fatoora_InvoiceSigner_certificate_pem_result fatoora_InvoiceSigner_certificate_pem(const InvoiceSigner* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceSigner_sign_result {union {SignedInvoice* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_sign_result;
fatoora_InvoiceSigner_sign_result fatoora_InvoiceSigner_sign(const InvoiceSigner* self, FinalizedInvoice* invoice);

typedef struct fatoora_InvoiceSigner_sign_xml_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_sign_xml_result;
fatoora_InvoiceSigner_sign_xml_result fatoora_InvoiceSigner_sign_xml(const InvoiceSigner* self, DiplomatStringView xml, DiplomatWrite* write);

void fatoora_InvoiceSigner_destroy(InvoiceSigner* self);





#endif // InvoiceSigner_H
