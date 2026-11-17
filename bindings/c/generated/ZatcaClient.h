#ifndef ZatcaClient_H
#define ZatcaClient_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "ClearanceMode.d.h"
#include "ComplianceCsid.d.h"
#include "ComplianceRequest.d.h"
#include "Config.d.h"
#include "Csr.d.h"
#include "InvoiceRequest.d.h"
#include "ProductionCsid.d.h"
#include "ProductionRequest.d.h"
#include "SignedInvoice.d.h"
#include "ValidationResponse.d.h"

#include "ZatcaClient.d.h"






typedef struct fatoora_ZatcaClient_new_result {union {ZatcaClient* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_new_result;
fatoora_ZatcaClient_new_result fatoora_ZatcaClient_new(const Config* config);

typedef struct fatoora_ZatcaClient_with_timeout_result {union {ZatcaClient* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_with_timeout_result;
fatoora_ZatcaClient_with_timeout_result fatoora_ZatcaClient_with_timeout(const Config* config, uint32_t timeout_ms);

typedef struct fatoora_ZatcaClient_request_compliance_csid_result {union {ComplianceCsid* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_request_compliance_csid_result;
fatoora_ZatcaClient_request_compliance_csid_result fatoora_ZatcaClient_request_compliance_csid(const ZatcaClient* self, const Csr* csr, DiplomatStringView otp);

typedef struct fatoora_ZatcaClient_request_production_csid_result {union {ProductionCsid* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_request_production_csid_result;
fatoora_ZatcaClient_request_production_csid_result fatoora_ZatcaClient_request_production_csid(const ZatcaClient* self, const ComplianceCsid* credentials);

typedef struct fatoora_ZatcaClient_renew_csid_result {union {ProductionCsid* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_renew_csid_result;
fatoora_ZatcaClient_renew_csid_result fatoora_ZatcaClient_renew_csid(const ZatcaClient* self, const ProductionCsid* credentials, const Csr* csr, DiplomatStringView otp, OptionStringView accept_language);

typedef struct fatoora_ZatcaClient_check_invoice_compliance_result {union {ValidationResponse* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_check_invoice_compliance_result;
fatoora_ZatcaClient_check_invoice_compliance_result fatoora_ZatcaClient_check_invoice_compliance(const ZatcaClient* self, const SignedInvoice* invoice, const ComplianceCsid* credentials);

typedef struct fatoora_ZatcaClient_report_simplified_invoice_result {union {ValidationResponse* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_report_simplified_invoice_result;
fatoora_ZatcaClient_report_simplified_invoice_result fatoora_ZatcaClient_report_simplified_invoice(const ZatcaClient* self, const SignedInvoice* invoice, const ProductionCsid* credentials, ClearanceMode clearance_mode, OptionStringView accept_language);

typedef struct fatoora_ZatcaClient_clear_standard_invoice_result {union {ValidationResponse* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_clear_standard_invoice_result;
fatoora_ZatcaClient_clear_standard_invoice_result fatoora_ZatcaClient_clear_standard_invoice(const ZatcaClient* self, const SignedInvoice* invoice, const ProductionCsid* credentials, ClearanceMode clearance_mode, OptionStringView accept_language);

typedef struct fatoora_ZatcaClient_prepare_request_compliance_csid_result {union {ComplianceRequest* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_request_compliance_csid_result;
fatoora_ZatcaClient_prepare_request_compliance_csid_result fatoora_ZatcaClient_prepare_request_compliance_csid(const ZatcaClient* self, const Csr* csr, DiplomatStringView otp);

typedef struct fatoora_ZatcaClient_prepare_request_production_csid_result {union {ProductionRequest* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_request_production_csid_result;
fatoora_ZatcaClient_prepare_request_production_csid_result fatoora_ZatcaClient_prepare_request_production_csid(const ZatcaClient* self, const ComplianceCsid* credentials);

typedef struct fatoora_ZatcaClient_prepare_renew_csid_result {union {ProductionRequest* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_renew_csid_result;
fatoora_ZatcaClient_prepare_renew_csid_result fatoora_ZatcaClient_prepare_renew_csid(const ZatcaClient* self, const ProductionCsid* credentials, const Csr* csr, DiplomatStringView otp, OptionStringView accept_language);

typedef struct fatoora_ZatcaClient_prepare_check_invoice_compliance_result {union {InvoiceRequest* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_check_invoice_compliance_result;
fatoora_ZatcaClient_prepare_check_invoice_compliance_result fatoora_ZatcaClient_prepare_check_invoice_compliance(const ZatcaClient* self, const SignedInvoice* invoice, const ComplianceCsid* credentials);

typedef struct fatoora_ZatcaClient_prepare_report_simplified_invoice_result {union {InvoiceRequest* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_report_simplified_invoice_result;
fatoora_ZatcaClient_prepare_report_simplified_invoice_result fatoora_ZatcaClient_prepare_report_simplified_invoice(const ZatcaClient* self, const SignedInvoice* invoice, const ProductionCsid* credentials, ClearanceMode clearance_mode, OptionStringView accept_language);

typedef struct fatoora_ZatcaClient_prepare_clear_standard_invoice_result {union {InvoiceRequest* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_clear_standard_invoice_result;
fatoora_ZatcaClient_prepare_clear_standard_invoice_result fatoora_ZatcaClient_prepare_clear_standard_invoice(const ZatcaClient* self, const SignedInvoice* invoice, const ProductionCsid* credentials, ClearanceMode clearance_mode, OptionStringView accept_language);

void fatoora_ZatcaClient_destroy(ZatcaClient* self);





#endif // ZatcaClient_H
