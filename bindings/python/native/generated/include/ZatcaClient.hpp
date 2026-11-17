#ifndef _NATIVE_ZatcaClient_HPP
#define _NATIVE_ZatcaClient_HPP

#include "ZatcaClient.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
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
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ZatcaClient_create_result {union {_native::capi::ZatcaClient* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_create_result;
    fatoora_ZatcaClient_create_result fatoora_ZatcaClient_create(const _native::capi::Config* config);

    typedef struct fatoora_ZatcaClient_request_compliance_csid_result {union {_native::capi::ComplianceCsid* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_request_compliance_csid_result;
    fatoora_ZatcaClient_request_compliance_csid_result fatoora_ZatcaClient_request_compliance_csid(const _native::capi::ZatcaClient* self, const _native::capi::Csr* csr, _native::diplomat::capi::DiplomatStringView otp);

    typedef struct fatoora_ZatcaClient_request_production_csid_result {union {_native::capi::ProductionCsid* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_request_production_csid_result;
    fatoora_ZatcaClient_request_production_csid_result fatoora_ZatcaClient_request_production_csid(const _native::capi::ZatcaClient* self, const _native::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_renew_csid_result {union {_native::capi::ProductionCsid* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_renew_csid_result;
    fatoora_ZatcaClient_renew_csid_result fatoora_ZatcaClient_renew_csid(const _native::capi::ZatcaClient* self, const _native::capi::ProductionCsid* credentials, const _native::capi::Csr* csr, _native::diplomat::capi::DiplomatStringView otp, _native::diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_check_invoice_compliance_result {union {_native::capi::ValidationResponse* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_check_invoice_compliance_result;
    fatoora_ZatcaClient_check_invoice_compliance_result fatoora_ZatcaClient_check_invoice_compliance(const _native::capi::ZatcaClient* self, const _native::capi::SignedInvoice* invoice, const _native::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_report_simplified_invoice_result {union {_native::capi::ValidationResponse* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_report_simplified_invoice_result;
    fatoora_ZatcaClient_report_simplified_invoice_result fatoora_ZatcaClient_report_simplified_invoice(const _native::capi::ZatcaClient* self, const _native::capi::SignedInvoice* invoice, const _native::capi::ProductionCsid* credentials, _native::capi::ClearanceMode clearance_mode, _native::diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_clear_standard_invoice_result {union {_native::capi::ValidationResponse* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_clear_standard_invoice_result;
    fatoora_ZatcaClient_clear_standard_invoice_result fatoora_ZatcaClient_clear_standard_invoice(const _native::capi::ZatcaClient* self, const _native::capi::SignedInvoice* invoice, const _native::capi::ProductionCsid* credentials, _native::capi::ClearanceMode clearance_mode, _native::diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_prepare_request_compliance_csid_result {union {_native::capi::ComplianceRequest* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_request_compliance_csid_result;
    fatoora_ZatcaClient_prepare_request_compliance_csid_result fatoora_ZatcaClient_prepare_request_compliance_csid(const _native::capi::ZatcaClient* self, const _native::capi::Csr* csr, _native::diplomat::capi::DiplomatStringView otp);

    typedef struct fatoora_ZatcaClient_prepare_request_production_csid_result {union {_native::capi::ProductionRequest* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_request_production_csid_result;
    fatoora_ZatcaClient_prepare_request_production_csid_result fatoora_ZatcaClient_prepare_request_production_csid(const _native::capi::ZatcaClient* self, const _native::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_prepare_renew_csid_result {union {_native::capi::ProductionRequest* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_renew_csid_result;
    fatoora_ZatcaClient_prepare_renew_csid_result fatoora_ZatcaClient_prepare_renew_csid(const _native::capi::ZatcaClient* self, const _native::capi::ProductionCsid* credentials, const _native::capi::Csr* csr, _native::diplomat::capi::DiplomatStringView otp, _native::diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_prepare_check_invoice_compliance_result {union {_native::capi::InvoiceRequest* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_check_invoice_compliance_result;
    fatoora_ZatcaClient_prepare_check_invoice_compliance_result fatoora_ZatcaClient_prepare_check_invoice_compliance(const _native::capi::ZatcaClient* self, const _native::capi::SignedInvoice* invoice, const _native::capi::ComplianceCsid* credentials);

    typedef struct fatoora_ZatcaClient_prepare_report_simplified_invoice_result {union {_native::capi::InvoiceRequest* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_report_simplified_invoice_result;
    fatoora_ZatcaClient_prepare_report_simplified_invoice_result fatoora_ZatcaClient_prepare_report_simplified_invoice(const _native::capi::ZatcaClient* self, const _native::capi::SignedInvoice* invoice, const _native::capi::ProductionCsid* credentials, _native::capi::ClearanceMode clearance_mode, _native::diplomat::capi::OptionStringView accept_language);

    typedef struct fatoora_ZatcaClient_prepare_clear_standard_invoice_result {union {_native::capi::InvoiceRequest* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaClient_prepare_clear_standard_invoice_result;
    fatoora_ZatcaClient_prepare_clear_standard_invoice_result fatoora_ZatcaClient_prepare_clear_standard_invoice(const _native::capi::ZatcaClient* self, const _native::capi::SignedInvoice* invoice, const _native::capi::ProductionCsid* credentials, _native::capi::ClearanceMode clearance_mode, _native::diplomat::capi::OptionStringView accept_language);

    void fatoora_ZatcaClient_destroy(ZatcaClient* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::ZatcaClient>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::create(const _native::Config& config) {
    auto result = _native::capi::fatoora_ZatcaClient_create(config.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ZatcaClient>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ZatcaClient>>(std::unique_ptr<_native::ZatcaClient>(_native::ZatcaClient::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ZatcaClient>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::request_compliance_csid(const _native::Csr& csr, std::string_view otp) const {
    auto result = _native::capi::fatoora_ZatcaClient_request_compliance_csid(this->AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ComplianceCsid>>(std::unique_ptr<_native::ComplianceCsid>(_native::ComplianceCsid::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::request_production_csid(const _native::ComplianceCsid& credentials) const {
    auto result = _native::capi::fatoora_ZatcaClient_request_production_csid(this->AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ProductionCsid>>(std::unique_ptr<_native::ProductionCsid>(_native::ProductionCsid::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::renew_csid(const _native::ProductionCsid& credentials, const _native::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const {
    auto result = _native::capi::fatoora_ZatcaClient_renew_csid(this->AsFFI(),
        credentials.AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()},
        accept_language.has_value() ? (_native::diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ProductionCsid>>(std::unique_ptr<_native::ProductionCsid>(_native::ProductionCsid::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::check_invoice_compliance(const _native::SignedInvoice& invoice, const _native::ComplianceCsid& credentials) const {
    auto result = _native::capi::fatoora_ZatcaClient_check_invoice_compliance(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ValidationResponse>>(std::unique_ptr<_native::ValidationResponse>(_native::ValidationResponse::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::report_simplified_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = _native::capi::fatoora_ZatcaClient_report_simplified_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (_native::diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ValidationResponse>>(std::unique_ptr<_native::ValidationResponse>(_native::ValidationResponse::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::clear_standard_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = _native::capi::fatoora_ZatcaClient_clear_standard_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (_native::diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ValidationResponse>>(std::unique_ptr<_native::ValidationResponse>(_native::ValidationResponse::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ComplianceRequest>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::prepare_request_compliance_csid(const _native::Csr& csr, std::string_view otp) const {
    auto result = _native::capi::fatoora_ZatcaClient_prepare_request_compliance_csid(this->AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ComplianceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ComplianceRequest>>(std::unique_ptr<_native::ComplianceRequest>(_native::ComplianceRequest::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ComplianceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::prepare_request_production_csid(const _native::ComplianceCsid& credentials) const {
    auto result = _native::capi::fatoora_ZatcaClient_prepare_request_production_csid(this->AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ProductionRequest>>(std::unique_ptr<_native::ProductionRequest>(_native::ProductionRequest::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::prepare_renew_csid(const _native::ProductionCsid& credentials, const _native::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const {
    auto result = _native::capi::fatoora_ZatcaClient_prepare_renew_csid(this->AsFFI(),
        credentials.AsFFI(),
        csr.AsFFI(),
        {otp.data(), otp.size()},
        accept_language.has_value() ? (_native::diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ProductionRequest>>(std::unique_ptr<_native::ProductionRequest>(_native::ProductionRequest::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::prepare_check_invoice_compliance(const _native::SignedInvoice& invoice, const _native::ComplianceCsid& credentials) const {
    auto result = _native::capi::fatoora_ZatcaClient_prepare_check_invoice_compliance(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceRequest>>(std::unique_ptr<_native::InvoiceRequest>(_native::InvoiceRequest::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::prepare_report_simplified_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = _native::capi::fatoora_ZatcaClient_prepare_report_simplified_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (_native::diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceRequest>>(std::unique_ptr<_native::InvoiceRequest>(_native::InvoiceRequest::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>> _native::ZatcaClient::prepare_clear_standard_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const {
    auto result = _native::capi::fatoora_ZatcaClient_prepare_clear_standard_invoice(this->AsFFI(),
        invoice.AsFFI(),
        credentials.AsFFI(),
        clearance_mode.AsFFI(),
        accept_language.has_value() ? (_native::diplomat::capi::OptionStringView{ { {accept_language.value().data(), accept_language.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceRequest>>(std::unique_ptr<_native::InvoiceRequest>(_native::InvoiceRequest::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ZatcaClient* _native::ZatcaClient::AsFFI() const {
    return reinterpret_cast<const _native::capi::ZatcaClient*>(this);
}

inline _native::capi::ZatcaClient* _native::ZatcaClient::AsFFI() {
    return reinterpret_cast<_native::capi::ZatcaClient*>(this);
}

inline const _native::ZatcaClient* _native::ZatcaClient::FromFFI(const _native::capi::ZatcaClient* ptr) {
    return reinterpret_cast<const _native::ZatcaClient*>(ptr);
}

inline _native::ZatcaClient* _native::ZatcaClient::FromFFI(_native::capi::ZatcaClient* ptr) {
    return reinterpret_cast<_native::ZatcaClient*>(ptr);
}

inline void _native::ZatcaClient::operator delete(void* ptr) {
    _native::capi::fatoora_ZatcaClient_destroy(reinterpret_cast<_native::capi::ZatcaClient*>(ptr));
}


#endif // _NATIVE_ZatcaClient_HPP
