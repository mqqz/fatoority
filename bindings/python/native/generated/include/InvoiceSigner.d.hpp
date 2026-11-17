#ifndef _NATIVE_InvoiceSigner_D_HPP
#define _NATIVE_InvoiceSigner_D_HPP

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
namespace capi { struct Bytes; }
class Bytes;
namespace capi { struct FinalizedInvoice; }
class FinalizedInvoice;
namespace capi { struct InvoiceSigner; }
class InvoiceSigner;
namespace capi { struct SignedInvoice; }
class SignedInvoice;
} // namespace _native



namespace _native {
namespace capi {
    struct InvoiceSigner;
} // namespace capi
} // namespace

namespace _native {
class InvoiceSigner {
public:

  inline static _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>> from_pem(std::string_view cert_pem, std::string_view key_pem);

  inline static _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>> from_der(_native::diplomat::span<const uint8_t> cert_der, _native::diplomat::span<const uint8_t> key_der);

  inline _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>> certificate_der() const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> certificate_pem() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> certificate_pem_write(W& writeable_output) const;

  inline _native::diplomat::result<std::unique_ptr<_native::SignedInvoice>, std::unique_ptr<_native::BindingError>> sign(_native::FinalizedInvoice& invoice) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> sign_xml(std::string_view xml) const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> sign_xml_write(std::string_view xml, W& writeable_output) const;

    inline const _native::capi::InvoiceSigner* AsFFI() const;
    inline _native::capi::InvoiceSigner* AsFFI();
    inline static const _native::InvoiceSigner* FromFFI(const _native::capi::InvoiceSigner* ptr);
    inline static _native::InvoiceSigner* FromFFI(_native::capi::InvoiceSigner* ptr);
    inline static void operator delete(void* ptr);
private:
    InvoiceSigner() = delete;
    InvoiceSigner(const _native::InvoiceSigner&) = delete;
    InvoiceSigner(_native::InvoiceSigner&&) noexcept = delete;
    InvoiceSigner operator=(const _native::InvoiceSigner&) = delete;
    InvoiceSigner operator=(_native::InvoiceSigner&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_InvoiceSigner_D_HPP
