#ifndef _NATIVE_ZatcaClient_D_HPP
#define _NATIVE_ZatcaClient_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace _native {
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct ComplianceCsid; }
class ComplianceCsid;
namespace capi { struct ComplianceRequest; }
class ComplianceRequest;
namespace capi { struct Config; }
class Config;
namespace capi { struct Csr; }
class Csr;
namespace capi { struct InvoiceRequest; }
class InvoiceRequest;
namespace capi { struct ProductionCsid; }
class ProductionCsid;
namespace capi { struct ProductionRequest; }
class ProductionRequest;
namespace capi { struct SignedInvoice; }
class SignedInvoice;
namespace capi { struct ValidationResponse; }
class ValidationResponse;
namespace capi { struct ZatcaClient; }
class ZatcaClient;
class ClearanceMode;
} // namespace _native



namespace _native {
namespace capi {
    struct ZatcaClient;
} // namespace capi
} // namespace

namespace _native {
class ZatcaClient {
public:

  inline static _native::diplomat::result<std::unique_ptr<_native::ZatcaClient>, std::unique_ptr<_native::BindingError>> create(const _native::Config& config);

  inline _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>> request_compliance_csid(const _native::Csr& csr, std::string_view otp) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> request_production_csid(const _native::ComplianceCsid& credentials) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> renew_csid(const _native::ProductionCsid& credentials, const _native::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> check_invoice_compliance(const _native::SignedInvoice& invoice, const _native::ComplianceCsid& credentials) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> report_simplified_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> clear_standard_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ComplianceRequest>, std::unique_ptr<_native::BindingError>> prepare_request_compliance_csid(const _native::Csr& csr, std::string_view otp) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>> prepare_request_production_csid(const _native::ComplianceCsid& credentials) const;

  inline _native::diplomat::result<std::unique_ptr<_native::ProductionRequest>, std::unique_ptr<_native::BindingError>> prepare_renew_csid(const _native::ProductionCsid& credentials, const _native::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const;

  inline _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>> prepare_check_invoice_compliance(const _native::SignedInvoice& invoice, const _native::ComplianceCsid& credentials) const;

  inline _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>> prepare_report_simplified_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

  inline _native::diplomat::result<std::unique_ptr<_native::InvoiceRequest>, std::unique_ptr<_native::BindingError>> prepare_clear_standard_invoice(const _native::SignedInvoice& invoice, const _native::ProductionCsid& credentials, _native::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

    inline const _native::capi::ZatcaClient* AsFFI() const;
    inline _native::capi::ZatcaClient* AsFFI();
    inline static const _native::ZatcaClient* FromFFI(const _native::capi::ZatcaClient* ptr);
    inline static _native::ZatcaClient* FromFFI(_native::capi::ZatcaClient* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaClient() = delete;
    ZatcaClient(const _native::ZatcaClient&) = delete;
    ZatcaClient(_native::ZatcaClient&&) noexcept = delete;
    ZatcaClient operator=(const _native::ZatcaClient&) = delete;
    ZatcaClient operator=(_native::ZatcaClient&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ZatcaClient_D_HPP
