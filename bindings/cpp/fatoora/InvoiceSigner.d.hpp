#ifndef fatoora_InvoiceSigner_D_HPP
#define fatoora_InvoiceSigner_D_HPP

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
namespace capi { struct Bytes; }
class Bytes;
namespace capi { struct FinalizedInvoice; }
class FinalizedInvoice;
namespace capi { struct InvoiceSigner; }
class InvoiceSigner;
namespace capi { struct SignedInvoice; }
class SignedInvoice;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct InvoiceSigner;
} // namespace capi
} // namespace

namespace fatoora {
class InvoiceSigner {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>> from_pem(std::string_view cert_pem, std::string_view key_pem);

  inline static diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>> from_der(diplomat::span<const uint8_t> cert_der, diplomat::span<const uint8_t> key_der);

  inline diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>> certificate_der() const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> certificate_pem() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> certificate_pem_write(W& writeable_output) const;

  inline diplomat::result<std::unique_ptr<fatoora::SignedInvoice>, std::unique_ptr<fatoora::BindingError>> sign(fatoora::FinalizedInvoice& invoice) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> sign_xml(std::string_view xml) const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> sign_xml_write(std::string_view xml, W& writeable_output) const;

    inline const fatoora::capi::InvoiceSigner* AsFFI() const;
    inline fatoora::capi::InvoiceSigner* AsFFI();
    inline static const fatoora::InvoiceSigner* FromFFI(const fatoora::capi::InvoiceSigner* ptr);
    inline static fatoora::InvoiceSigner* FromFFI(fatoora::capi::InvoiceSigner* ptr);
    inline static void operator delete(void* ptr);
private:
    InvoiceSigner() = delete;
    InvoiceSigner(const fatoora::InvoiceSigner&) = delete;
    InvoiceSigner(fatoora::InvoiceSigner&&) noexcept = delete;
    InvoiceSigner operator=(const fatoora::InvoiceSigner&) = delete;
    InvoiceSigner operator=(fatoora::InvoiceSigner&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_InvoiceSigner_D_HPP
