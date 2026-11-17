#ifndef fatoora_ZatcaClient_HPP
#define fatoora_ZatcaClient_HPP

#include "ZatcaClient.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "BindingError.hpp"
#include "ClearanceMode.hpp"
#include "ComplianceCsid.hpp"
#include "ComplianceRequest.hpp"
#include "Config.hpp"
#include "Csr.hpp"
#include "InvoiceRequest.hpp"
#include "ProductionCsid.hpp"
#include "ProductionRequest.hpp"
#include "SignedInvoice.hpp"
#include "ValidationResponse.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_ZatcaClient_new_result {union {fatoora::capi::ZatcaClient* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_new_result;
    fatoora_ZatcaClient_new_result fatoora_ZatcaClient_new(const fatoora::capi::Config* config);

    typedef struct fatoora_ZatcaClient_with_timeout_result {union {fatoora::capi::ZatcaClient* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_with_timeout_result;
    fatoora_ZatcaClient_with_timeout_result fatoora_ZatcaClient_with_timeout(const fatoora::capi::Config* config, uint32_t timeout_ms);

    typedef struct fatoora_ZatcaClient_request_compliance_csid_result {union {fatoora::capi::ComplianceCsid* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_request_compliance_csid_result;
    fatoora_ZatcaClient_request_compliance_csid_result fatoora_ZatcaClient_request_compliance_csid(const fatoora::capi::ZatcaClient* self, const fatoora::capi::Csr* csr, diplomat::capi::DiplomatStringView otp);

    typedef struct fatoora_ZatcaClient_request_production_csid_result {union {fatoora::capi::ProductionCsid* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_request_production_csid_result;
    fatoora_ZatcaClient_request_production_csid_result fatoora_ZatcaClient_request_production_csid(const fatoora::capi::ZatcaClient* self, const fatoora::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_renew_csid_result {union {fatoora::capi::ProductionCsid* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_renew_csid_result;
    fatoora_ZatcaClient_renew_csid_result fatoora_ZatcaClient_renew_csid(const fatoora::capi::ZatcaClient* self, const fatoora::capi::ProductionCsid* credentials, const fatoora::capi::Csr* csr, diplomat::capi::DiplomatStringView otp, diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_check_invoice_compliance_result {union {fatoora::capi::ValidationResponse* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_check_invoice_compliance_result;
    fatoora_ZatcaClient_check_invoice_compliance_result fatoora_ZatcaClient_check_invoice_compliance(const fatoora::capi::ZatcaClient* self, const fatoora::capi::SignedInvoice* invoice, const fatoora::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_report_simplified_invoice_result {union {fatoora::capi::ValidationResponse* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_report_simplified_invoice_result;
    fatoora_ZatcaClient_report_simplified_invoice_result fatoora_ZatcaClient_report_simplified_invoice(const fatoora::capi::ZatcaClient* self, const fatoora::capi::SignedInvoice* invoice, const fatoora::capi::ProductionCsid* credentials, fatoora::capi::ClearanceMode clearance_mode, diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_clear_standard_invoice_result {union {fatoora::capi::ValidationResponse* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_clear_standard_invoice_result;
    fatoora_ZatcaClient_clear_standard_invoice_result fatoora_ZatcaClient_clear_standard_invoice(const fatoora::capi::ZatcaClient* self, const fatoora::capi::SignedInvoice* invoice, const fatoora::capi::ProductionCsid* credentials, fatoora::capi::ClearanceMode clearance_mode, diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_prepare_request_compliance_csid_result {union {fatoora::capi::ComplianceRequest* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_request_compliance_csid_result;
    fatoora_ZatcaClient_prepare_request_compliance_csid_result fatoora_ZatcaClient_prepare_request_compliance_csid(const fatoora::capi::ZatcaClient* self, const fatoora::capi::Csr* csr, diplomat::capi::DiplomatStringView otp);

    typedef struct fatoora_ZatcaClient_prepare_request_production_csid_result {union {fatoora::capi::ProductionRequest* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_request_production_csid_result;
    fatoora_ZatcaClient_prepare_request_production_csid_result fatoora_ZatcaClient_prepare_request_production_csid(const fatoora::capi::ZatcaClient* self, const fatoora::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_prepare_renew_csid_result {union {fatoora::capi::ProductionRequest* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_renew_csid_result;
    fatoora_ZatcaClient_prepare_renew_csid_result fatoora_ZatcaClient_prepare_renew_csid(const fatoora::capi::ZatcaClient* self, const fatoora::capi::ProductionCsid* credentials, const fatoora::capi::Csr* csr, diplomat::capi::DiplomatStringView otp, diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_prepare_check_invoice_compliance_result {union {fatoora::capi::InvoiceRequest* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_check_invoice_compliance_result;
    fatoora_ZatcaClient_prepare_check_invoice_compliance_result fatoora_ZatcaClient_prepare_check_invoice_compliance(const fatoora::capi::ZatcaClient* self, const fatoora::capi::SignedInvoice* invoice, const fatoora::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_prepare_report_simplified_invoice_result {union {fatoora::capi::InvoiceRequest* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_report_simplified_invoice_result;
    fatoora_ZatcaClient_prepare_report_simplified_invoice_result fatoora_ZatcaClient_prepare_report_simplified_invoice(const fatoora::capi::ZatcaClient* self, const fatoora::capi::SignedInvoice* invoice, const fatoora::capi::ProductionCsid* credentials, fatoora::capi::ClearanceMode clearance_mode, diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_prepare_clear_standard_invoice_result {union {fatoora::capi::InvoiceRequest* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_clear_standard_invoice_result;
    fatoora_ZatcaClient_prepare_clear_standard_invoice_result fatoora_ZatcaClient_prepare_clear_standard_invoice(const fatoora::capi::ZatcaClient* self, const fatoora::capi::SignedInvoice* invoice, const fatoora::capi::ProductionCsid* credentials, fatoora::capi::ClearanceMode clearance_mode, diplomat::capi::OptionStringView accept_language);

    void fatoora_ZatcaClient_destroy(ZatcaClient* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::new_(const fatoora::Config& config) {
    auto result = fatoora::capi::fatoora_ZatcaClient_new(config.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ZatcaClient>>(std::unique_ptr<fatoora::ZatcaClient>(fatoora::ZatcaClient::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::with_timeout(const fatoora::Config& config, uint32_t timeout_ms) {
    auto result = fatoora::capi::fatoora_ZatcaClient_with_timeout(config.AsFFI(),
        timeout_ms);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ZatcaClient>>(std::unique_ptr<fatoora::ZatcaClient>(fatoora::ZatcaClient::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::request_compliance_csid(const fatoora::Csr& csr, std::string_view otp) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_request_compliance_csid(this->AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ComplianceCsid>>(std::unique_ptr<fatoora::ComplianceCsid>(fatoora::ComplianceCsid::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::request_production_csid(const fatoora::ComplianceCsid& credentials) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_request_production_csid(this->AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ProductionCsid>>(std::unique_ptr<fatoora::ProductionCsid>(fatoora::ProductionCsid::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::renew_csid(const fatoora::ProductionCsid& credentials, const fatoora::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_renew_csid(this->AsFFI(),
        credentials.AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()},
        accept_language.has_value() ? (diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ProductionCsid>>(std::unique_ptr<fatoora::ProductionCsid>(fatoora::ProductionCsid::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::check_invoice_compliance(const fatoora::SignedInvoice& invoice, const fatoora::ComplianceCsid& credentials) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_check_invoice_compliance(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ValidationResponse>>(std::unique_ptr<fatoora::ValidationResponse>(fatoora::ValidationResponse::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::report_simplified_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_report_simplified_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ValidationResponse>>(std::unique_ptr<fatoora::ValidationResponse>(fatoora::ValidationResponse::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::clear_standard_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_clear_standard_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ValidationResponse>>(std::unique_ptr<fatoora::ValidationResponse>(fatoora::ValidationResponse::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ComplianceRequest>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::prepare_request_compliance_csid(const fatoora::Csr& csr, std::string_view otp) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_prepare_request_compliance_csid(this->AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ComplianceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ComplianceRequest>>(std::unique_ptr<fatoora::ComplianceRequest>(fatoora::ComplianceRequest::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ComplianceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::prepare_request_production_csid(const fatoora::ComplianceCsid& credentials) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_prepare_request_production_csid(this->AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ProductionRequest>>(std::unique_ptr<fatoora::ProductionRequest>(fatoora::ProductionRequest::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::prepare_renew_csid(const fatoora::ProductionCsid& credentials, const fatoora::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_prepare_renew_csid(this->AsFFI(),
        credentials.AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()},
        accept_language.has_value() ? (diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ProductionRequest>>(std::unique_ptr<fatoora::ProductionRequest>(fatoora::ProductionRequest::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::prepare_check_invoice_compliance(const fatoora::SignedInvoice& invoice, const fatoora::ComplianceCsid& credentials) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_prepare_check_invoice_compliance(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceRequest>>(std::unique_ptr<fatoora::InvoiceRequest>(fatoora::InvoiceRequest::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::prepare_report_simplified_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_prepare_report_simplified_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceRequest>>(std::unique_ptr<fatoora::InvoiceRequest>(fatoora::InvoiceRequest::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaClient::prepare_clear_standard_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = fatoora::capi::fatoora_ZatcaClient_prepare_clear_standard_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceRequest>>(std::unique_ptr<fatoora::InvoiceRequest>(fatoora::InvoiceRequest::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ZatcaClient* fatoora::ZatcaClient::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ZatcaClient*>(this);
}

inline fatoora::capi::ZatcaClient* fatoora::ZatcaClient::AsFFI() {
    return reinterpret_cast<fatoora::capi::ZatcaClient*>(this);
}

inline const fatoora::ZatcaClient* fatoora::ZatcaClient::FromFFI(const fatoora::capi::ZatcaClient* ptr) {
    return reinterpret_cast<const fatoora::ZatcaClient*>(ptr);
}

inline fatoora::ZatcaClient* fatoora::ZatcaClient::FromFFI(fatoora::capi::ZatcaClient* ptr) {
    return reinterpret_cast<fatoora::ZatcaClient*>(ptr);
}

inline void fatoora::ZatcaClient::operator delete(void* ptr) {
    fatoora::capi::fatoora_ZatcaClient_destroy(reinterpret_cast<fatoora::capi::ZatcaClient*>(ptr));
}


#endif // fatoora_ZatcaClient_HPP
