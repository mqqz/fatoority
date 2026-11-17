#ifndef _NATIVE_ZatcaValidationReport_HPP
#define _NATIVE_ZatcaValidationReport_HPP

#include "ZatcaValidationReport.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "ZatcaStageReport.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    uint32_t fatoora_ZatcaValidationReport_schema_version(const _native::capi::ZatcaValidationReport* self);

    typedef struct fatoora_ZatcaValidationReport_profile_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_profile_result;
    fatoora_ZatcaValidationReport_profile_result fatoora_ZatcaValidationReport_profile(const _native::capi::ZatcaValidationReport* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ZatcaValidationReport_evaluated_at_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_evaluated_at_result;
    fatoora_ZatcaValidationReport_evaluated_at_result fatoora_ZatcaValidationReport_evaluated_at(const _native::capi::ZatcaValidationReport* self, _native::diplomat::capi::DiplomatWrite* write);

    bool fatoora_ZatcaValidationReport_is_complete(const _native::capi::ZatcaValidationReport* self);

    bool fatoora_ZatcaValidationReport_has_errors(const _native::capi::ZatcaValidationReport* self);

    bool fatoora_ZatcaValidationReport_is_valid(const _native::capi::ZatcaValidationReport* self);

    size_t fatoora_ZatcaValidationReport_stages_len(const _native::capi::ZatcaValidationReport* self);

    typedef struct fatoora_ZatcaValidationReport_stage_result {union {_native::capi::ZatcaStageReport* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_stage_result;
    fatoora_ZatcaValidationReport_stage_result fatoora_ZatcaValidationReport_stage(const _native::capi::ZatcaValidationReport* self, size_t index);

    typedef struct fatoora_ZatcaValidationReport_to_json_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_to_json_result;
    fatoora_ZatcaValidationReport_to_json_result fatoora_ZatcaValidationReport_to_json(const _native::capi::ZatcaValidationReport* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_ZatcaValidationReport_destroy(ZatcaValidationReport* self);

    } // extern "C"
} // namespace capi
} // namespace

inline uint32_t _native::ZatcaValidationReport::schema_version() const {
    auto result = _native::capi::fatoora_ZatcaValidationReport_schema_version(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ZatcaValidationReport::profile() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ZatcaValidationReport_profile(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ZatcaValidationReport::profile_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ZatcaValidationReport_profile(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ZatcaValidationReport::evaluated_at() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ZatcaValidationReport_evaluated_at(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ZatcaValidationReport::evaluated_at_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ZatcaValidationReport_evaluated_at(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline bool _native::ZatcaValidationReport::is_complete() const {
    auto result = _native::capi::fatoora_ZatcaValidationReport_is_complete(this->AsFFI());
    return result;
}

inline bool _native::ZatcaValidationReport::has_errors() const {
    auto result = _native::capi::fatoora_ZatcaValidationReport_has_errors(this->AsFFI());
    return result;
}

inline bool _native::ZatcaValidationReport::is_valid() const {
    auto result = _native::capi::fatoora_ZatcaValidationReport_is_valid(this->AsFFI());
    return result;
}

inline size_t _native::ZatcaValidationReport::stages_len() const {
    auto result = _native::capi::fatoora_ZatcaValidationReport_stages_len(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::ZatcaStageReport>, std::unique_ptr<_native::BindingError>> _native::ZatcaValidationReport::stage(size_t index) const {
    auto result = _native::capi::fatoora_ZatcaValidationReport_stage(this->AsFFI(),
        index);
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ZatcaStageReport>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ZatcaStageReport>>(std::unique_ptr<_native::ZatcaStageReport>(_native::ZatcaStageReport::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ZatcaStageReport>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ZatcaValidationReport::to_json() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ZatcaValidationReport_to_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ZatcaValidationReport::to_json_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ZatcaValidationReport_to_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ZatcaValidationReport* _native::ZatcaValidationReport::AsFFI() const {
    return reinterpret_cast<const _native::capi::ZatcaValidationReport*>(this);
}

inline _native::capi::ZatcaValidationReport* _native::ZatcaValidationReport::AsFFI() {
    return reinterpret_cast<_native::capi::ZatcaValidationReport*>(this);
}

inline const _native::ZatcaValidationReport* _native::ZatcaValidationReport::FromFFI(const _native::capi::ZatcaValidationReport* ptr) {
    return reinterpret_cast<const _native::ZatcaValidationReport*>(ptr);
}

inline _native::ZatcaValidationReport* _native::ZatcaValidationReport::FromFFI(_native::capi::ZatcaValidationReport* ptr) {
    return reinterpret_cast<_native::ZatcaValidationReport*>(ptr);
}

inline void _native::ZatcaValidationReport::operator delete(void* ptr) {
    _native::capi::fatoora_ZatcaValidationReport_destroy(reinterpret_cast<_native::capi::ZatcaValidationReport*>(ptr));
}


#endif // _NATIVE_ZatcaValidationReport_HPP
