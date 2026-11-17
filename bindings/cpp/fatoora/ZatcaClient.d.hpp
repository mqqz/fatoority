#ifndef fatoora_ZatcaClient_D_HPP
#define fatoora_ZatcaClient_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace fatoora {
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
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ZatcaClient;
} // namespace capi
} // namespace

namespace fatoora {
class ZatcaClient {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>> create(const fatoora::Config& config);

  /**
   * Like `create`, but each request fails with a network error after
   * the given number of milliseconds. Zero is rejected.
   */
  inline static diplomat::result<std::unique_ptr<fatoora::ZatcaClient>, std::unique_ptr<fatoora::BindingError>> create_with_timeout(const fatoora::Config& config, uint32_t timeout_ms);

  inline diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>> request_compliance_csid(const fatoora::Csr& csr, std::string_view otp) const;

  inline diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> request_production_csid(const fatoora::ComplianceCsid& credentials) const;

  inline diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> renew_csid(const fatoora::ProductionCsid& credentials, const fatoora::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const;

  inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> check_invoice_compliance(const fatoora::SignedInvoice& invoice, const fatoora::ComplianceCsid& credentials) const;

  inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> report_simplified_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

  inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> clear_standard_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

  inline diplomat::result<std::unique_ptr<fatoora::ComplianceRequest>, std::unique_ptr<fatoora::BindingError>> prepare_request_compliance_csid(const fatoora::Csr& csr, std::string_view otp) const;

  inline diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>> prepare_request_production_csid(const fatoora::ComplianceCsid& credentials) const;

  inline diplomat::result<std::unique_ptr<fatoora::ProductionRequest>, std::unique_ptr<fatoora::BindingError>> prepare_renew_csid(const fatoora::ProductionCsid& credentials, const fatoora::Csr& csr, std::string_view otp, std::optional<std::string_view> accept_language) const;

  inline diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>> prepare_check_invoice_compliance(const fatoora::SignedInvoice& invoice, const fatoora::ComplianceCsid& credentials) const;

  inline diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>> prepare_report_simplified_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

  inline diplomat::result<std::unique_ptr<fatoora::InvoiceRequest>, std::unique_ptr<fatoora::BindingError>> prepare_clear_standard_invoice(const fatoora::SignedInvoice& invoice, const fatoora::ProductionCsid& credentials, fatoora::ClearanceMode clearance_mode, std::optional<std::string_view> accept_language) const;

    inline const fatoora::capi::ZatcaClient* AsFFI() const;
    inline fatoora::capi::ZatcaClient* AsFFI();
    inline static const fatoora::ZatcaClient* FromFFI(const fatoora::capi::ZatcaClient* ptr);
    inline static fatoora::ZatcaClient* FromFFI(fatoora::capi::ZatcaClient* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaClient() = delete;
    ZatcaClient(const fatoora::ZatcaClient&) = delete;
    ZatcaClient(fatoora::ZatcaClient&&) noexcept = delete;
    ZatcaClient operator=(const fatoora::ZatcaClient&) = delete;
    ZatcaClient operator=(fatoora::ZatcaClient&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ZatcaClient_D_HPP
