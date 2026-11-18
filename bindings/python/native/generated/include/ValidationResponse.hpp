#ifndef _NATIVE_ValidationResponse_HPP
#define _NATIVE_ValidationResponse_HPP

#include "ValidationResponse.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "ClearanceStatus.hpp"
#include "InvoiceOutcome.hpp"
#include "ReportingStatus.hpp"
#include "ValidationResults.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ValidationResponse_http_status_result {union {uint16_t ok; }; bool is_ok;} fatoora_ValidationResponse_http_status_result;
    fatoora_ValidationResponse_http_status_result fatoora_ValidationResponse_http_status(const _native::capi::ValidationResponse* self);

    _native::capi::InvoiceOutcome fatoora_ValidationResponse_outcome(const _native::capi::ValidationResponse* self);

    typedef struct fatoora_ValidationResponse_ensure_accepted_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ValidationResponse_ensure_accepted_result;
    fatoora_ValidationResponse_ensure_accepted_result fatoora_ValidationResponse_ensure_accepted(const _native::capi::ValidationResponse* self);

    bool fatoora_ValidationResponse_cleared_invoice_xml_present(const _native::capi::ValidationResponse* self);

    typedef struct fatoora_ValidationResponse_cleared_invoice_xml_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ValidationResponse_cleared_invoice_xml_result;
    fatoora_ValidationResponse_cleared_invoice_xml_result fatoora_ValidationResponse_cleared_invoice_xml(const _native::capi::ValidationResponse* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationResponse_validation_results_result {union {_native::capi::ValidationResults* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ValidationResponse_validation_results_result;
    fatoora_ValidationResponse_validation_results_result fatoora_ValidationResponse_validation_results(const _native::capi::ValidationResponse* self);

    typedef struct fatoora_ValidationResponse_cleared_invoice_base64_result { bool is_ok;} fatoora_ValidationResponse_cleared_invoice_base64_result;
    fatoora_ValidationResponse_cleared_invoice_base64_result fatoora_ValidationResponse_cleared_invoice_base64(const _native::capi::ValidationResponse* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationResponse_reporting_status_result {union {_native::capi::ReportingStatus ok; }; bool is_ok;} fatoora_ValidationResponse_reporting_status_result;
    fatoora_ValidationResponse_reporting_status_result fatoora_ValidationResponse_reporting_status(const _native::capi::ValidationResponse* self);

    typedef struct fatoora_ValidationResponse_reporting_status_raw_result { bool is_ok;} fatoora_ValidationResponse_reporting_status_raw_result;
    fatoora_ValidationResponse_reporting_status_raw_result fatoora_ValidationResponse_reporting_status_raw(const _native::capi::ValidationResponse* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationResponse_clearance_status_result {union {_native::capi::ClearanceStatus ok; }; bool is_ok;} fatoora_ValidationResponse_clearance_status_result;
    fatoora_ValidationResponse_clearance_status_result fatoora_ValidationResponse_clearance_status(const _native::capi::ValidationResponse* self);

    typedef struct fatoora_ValidationResponse_clearance_status_raw_result { bool is_ok;} fatoora_ValidationResponse_clearance_status_raw_result;
    fatoora_ValidationResponse_clearance_status_raw_result fatoora_ValidationResponse_clearance_status_raw(const _native::capi::ValidationResponse* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationResponse_qr_seller_status_result { bool is_ok;} fatoora_ValidationResponse_qr_seller_status_result;
    fatoora_ValidationResponse_qr_seller_status_result fatoora_ValidationResponse_qr_seller_status(const _native::capi::ValidationResponse* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationResponse_qr_buyer_status_result { bool is_ok;} fatoora_ValidationResponse_qr_buyer_status_result;
    fatoora_ValidationResponse_qr_buyer_status_result fatoora_ValidationResponse_qr_buyer_status(const _native::capi::ValidationResponse* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_ValidationResponse_destroy(ValidationResponse* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::optional<uint16_t> _native::ValidationResponse::http_status() const {
    auto result = _native::capi::fatoora_ValidationResponse_http_status(this->AsFFI());
    return result.is_ok ? std::optional<uint16_t>(result.ok) : std::nullopt;
}

inline _native::InvoiceOutcome _native::ValidationResponse::outcome() const {
    auto result = _native::capi::fatoora_ValidationResponse_outcome(this->AsFFI());
    return _native::InvoiceOutcome::FromFFI(result);
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ValidationResponse::ensure_accepted() const {
    auto result = _native::capi::fatoora_ValidationResponse_ensure_accepted(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline bool _native::ValidationResponse::cleared_invoice_xml_present() const {
    auto result = _native::capi::fatoora_ValidationResponse_cleared_invoice_xml_present(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ValidationResponse::cleared_invoice_xml() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationResponse_cleared_invoice_xml(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ValidationResponse::cleared_invoice_xml_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationResponse_cleared_invoice_xml(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ValidationResults>, std::unique_ptr<_native::BindingError>> _native::ValidationResponse::validation_results() const {
    auto result = _native::capi::fatoora_ValidationResponse_validation_results(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ValidationResults>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ValidationResults>>(std::unique_ptr<_native::ValidationResults>(_native::ValidationResults::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ValidationResults>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> _native::ValidationResponse::cleared_invoice_base64() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationResponse_cleared_invoice_base64(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationResponse::cleared_invoice_base64_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationResponse_cleared_invoice_base64(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<_native::ReportingStatus> _native::ValidationResponse::reporting_status() const {
    auto result = _native::capi::fatoora_ValidationResponse_reporting_status(this->AsFFI());
    return result.is_ok ? std::optional<_native::ReportingStatus>(_native::ReportingStatus::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationResponse::reporting_status_raw() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationResponse_reporting_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationResponse::reporting_status_raw_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationResponse_reporting_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<_native::ClearanceStatus> _native::ValidationResponse::clearance_status() const {
    auto result = _native::capi::fatoora_ValidationResponse_clearance_status(this->AsFFI());
    return result.is_ok ? std::optional<_native::ClearanceStatus>(_native::ClearanceStatus::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationResponse::clearance_status_raw() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationResponse_clearance_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationResponse::clearance_status_raw_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationResponse_clearance_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationResponse::qr_seller_status() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationResponse_qr_seller_status(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationResponse::qr_seller_status_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationResponse_qr_seller_status(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationResponse::qr_buyer_status() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationResponse_qr_buyer_status(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationResponse::qr_buyer_status_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationResponse_qr_buyer_status(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline const _native::capi::ValidationResponse* _native::ValidationResponse::AsFFI() const {
    return reinterpret_cast<const _native::capi::ValidationResponse*>(this);
}

inline _native::capi::ValidationResponse* _native::ValidationResponse::AsFFI() {
    return reinterpret_cast<_native::capi::ValidationResponse*>(this);
}

inline const _native::ValidationResponse* _native::ValidationResponse::FromFFI(const _native::capi::ValidationResponse* ptr) {
    return reinterpret_cast<const _native::ValidationResponse*>(ptr);
}

inline _native::ValidationResponse* _native::ValidationResponse::FromFFI(_native::capi::ValidationResponse* ptr) {
    return reinterpret_cast<_native::ValidationResponse*>(ptr);
}

inline void _native::ValidationResponse::operator delete(void* ptr) {
    _native::capi::fatoora_ValidationResponse_destroy(reinterpret_cast<_native::capi::ValidationResponse*>(ptr));
}


#endif // _NATIVE_ValidationResponse_HPP
