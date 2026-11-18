#ifndef _NATIVE_OriginalInvoiceRef_HPP
#define _NATIVE_OriginalInvoiceRef_HPP

#include "OriginalInvoiceRef.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_OriginalInvoiceRef_new_result {union {_native::capi::OriginalInvoiceRef* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_OriginalInvoiceRef_new_result;
    fatoora_OriginalInvoiceRef_new_result fatoora_OriginalInvoiceRef_new(_native::diplomat::capi::DiplomatStringView id, _native::diplomat::capi::OptionStringView uuid, _native::diplomat::capi::OptionStringView date, _native::diplomat::capi::OptionStringView time, _native::diplomat::capi::OptionStringView type_code);

    typedef struct fatoora_OriginalInvoiceRef_issue_time_result { bool is_ok;} fatoora_OriginalInvoiceRef_issue_time_result;
    fatoora_OriginalInvoiceRef_issue_time_result fatoora_OriginalInvoiceRef_issue_time(const _native::capi::OriginalInvoiceRef* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_OriginalInvoiceRef_document_type_code_result { bool is_ok;} fatoora_OriginalInvoiceRef_document_type_code_result;
    fatoora_OriginalInvoiceRef_document_type_code_result fatoora_OriginalInvoiceRef_document_type_code(const _native::capi::OriginalInvoiceRef* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_OriginalInvoiceRef_id_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_OriginalInvoiceRef_id_result;
    fatoora_OriginalInvoiceRef_id_result fatoora_OriginalInvoiceRef_id(const _native::capi::OriginalInvoiceRef* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_OriginalInvoiceRef_uuid_result { bool is_ok;} fatoora_OriginalInvoiceRef_uuid_result;
    fatoora_OriginalInvoiceRef_uuid_result fatoora_OriginalInvoiceRef_uuid(const _native::capi::OriginalInvoiceRef* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_OriginalInvoiceRef_issue_date_result { bool is_ok;} fatoora_OriginalInvoiceRef_issue_date_result;
    fatoora_OriginalInvoiceRef_issue_date_result fatoora_OriginalInvoiceRef_issue_date(const _native::capi::OriginalInvoiceRef* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_OriginalInvoiceRef_destroy(OriginalInvoiceRef* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::OriginalInvoiceRef>, std::unique_ptr<_native::BindingError>> _native::OriginalInvoiceRef::new_(std::string_view id, std::optional<std::string_view> uuid, std::optional<std::string_view> date, std::optional<std::string_view> time, std::optional<std::string_view> type_code) {
    auto result = _native::capi::fatoora_OriginalInvoiceRef_new({id.data(), id.size()},
        uuid.has_value() ? (_native::diplomat::capi::OptionStringView{ { {uuid.value().data(), uuid.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        date.has_value() ? (_native::diplomat::capi::OptionStringView{ { {date.value().data(), date.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        time.has_value() ? (_native::diplomat::capi::OptionStringView{ { {time.value().data(), time.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        type_code.has_value() ? (_native::diplomat::capi::OptionStringView{ { {type_code.value().data(), type_code.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::OriginalInvoiceRef>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::OriginalInvoiceRef>>(std::unique_ptr<_native::OriginalInvoiceRef>(_native::OriginalInvoiceRef::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::OriginalInvoiceRef>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> _native::OriginalInvoiceRef::issue_time() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_issue_time(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::OriginalInvoiceRef::issue_time_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_issue_time(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::OriginalInvoiceRef::document_type_code() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_document_type_code(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::OriginalInvoiceRef::document_type_code_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_document_type_code(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::OriginalInvoiceRef::id() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_id(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::OriginalInvoiceRef::id_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_id(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> _native::OriginalInvoiceRef::uuid() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_uuid(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::OriginalInvoiceRef::uuid_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_uuid(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::OriginalInvoiceRef::issue_date() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_issue_date(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::OriginalInvoiceRef::issue_date_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_OriginalInvoiceRef_issue_date(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline const _native::capi::OriginalInvoiceRef* _native::OriginalInvoiceRef::AsFFI() const {
    return reinterpret_cast<const _native::capi::OriginalInvoiceRef*>(this);
}

inline _native::capi::OriginalInvoiceRef* _native::OriginalInvoiceRef::AsFFI() {
    return reinterpret_cast<_native::capi::OriginalInvoiceRef*>(this);
}

inline const _native::OriginalInvoiceRef* _native::OriginalInvoiceRef::FromFFI(const _native::capi::OriginalInvoiceRef* ptr) {
    return reinterpret_cast<const _native::OriginalInvoiceRef*>(ptr);
}

inline _native::OriginalInvoiceRef* _native::OriginalInvoiceRef::FromFFI(_native::capi::OriginalInvoiceRef* ptr) {
    return reinterpret_cast<_native::OriginalInvoiceRef*>(ptr);
}

inline void _native::OriginalInvoiceRef::operator delete(void* ptr) {
    _native::capi::fatoora_OriginalInvoiceRef_destroy(reinterpret_cast<_native::capi::OriginalInvoiceRef*>(ptr));
}


#endif // _NATIVE_OriginalInvoiceRef_HPP
