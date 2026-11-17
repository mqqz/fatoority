#ifndef _NATIVE_ZatcaStageReport_HPP
#define _NATIVE_ZatcaStageReport_HPP

#include "ZatcaStageReport.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "Text.hpp"
#include "ZatcaFinding.hpp"
#include "ZatcaRuleSource.hpp"
#include "ZatcaStage.hpp"
#include "ZatcaStageStatus.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    _native::capi::ZatcaStage fatoora_ZatcaStageReport_stage(const _native::capi::ZatcaStageReport* self);

    _native::capi::ZatcaStageStatus fatoora_ZatcaStageReport_status(const _native::capi::ZatcaStageReport* self);

    typedef struct fatoora_ZatcaStageReport_provenance_result {union {_native::capi::ZatcaRuleSource* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_provenance_result;
    fatoora_ZatcaStageReport_provenance_result fatoora_ZatcaStageReport_provenance(const _native::capi::ZatcaStageReport* self);

    size_t fatoora_ZatcaStageReport_evaluated_assertions_len(const _native::capi::ZatcaStageReport* self);

    typedef struct fatoora_ZatcaStageReport_evaluated_assertion_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_evaluated_assertion_result;
    fatoora_ZatcaStageReport_evaluated_assertion_result fatoora_ZatcaStageReport_evaluated_assertion(const _native::capi::ZatcaStageReport* self, size_t index);

    size_t fatoora_ZatcaStageReport_findings_len(const _native::capi::ZatcaStageReport* self);

    typedef struct fatoora_ZatcaStageReport_finding_result {union {_native::capi::ZatcaFinding* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_finding_result;
    fatoora_ZatcaStageReport_finding_result fatoora_ZatcaStageReport_finding(const _native::capi::ZatcaStageReport* self, size_t index);

    void fatoora_ZatcaStageReport_destroy(ZatcaStageReport* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::ZatcaStage _native::ZatcaStageReport::stage() const {
    auto result = _native::capi::fatoora_ZatcaStageReport_stage(this->AsFFI());
    return _native::ZatcaStage::FromFFI(result);
}

inline _native::ZatcaStageStatus _native::ZatcaStageReport::status() const {
    auto result = _native::capi::fatoora_ZatcaStageReport_status(this->AsFFI());
    return _native::ZatcaStageStatus::FromFFI(result);
}

inline _native::diplomat::result<std::unique_ptr<_native::ZatcaRuleSource>, std::unique_ptr<_native::BindingError>> _native::ZatcaStageReport::provenance() const {
    auto result = _native::capi::fatoora_ZatcaStageReport_provenance(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ZatcaRuleSource>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ZatcaRuleSource>>(std::unique_ptr<_native::ZatcaRuleSource>(_native::ZatcaRuleSource::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ZatcaRuleSource>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline size_t _native::ZatcaStageReport::evaluated_assertions_len() const {
    auto result = _native::capi::fatoora_ZatcaStageReport_evaluated_assertions_len(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::ZatcaStageReport::evaluated_assertion(size_t index) const {
    auto result = _native::capi::fatoora_ZatcaStageReport_evaluated_assertion(this->AsFFI(),
        index);
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline size_t _native::ZatcaStageReport::findings_len() const {
    auto result = _native::capi::fatoora_ZatcaStageReport_findings_len(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::ZatcaFinding>, std::unique_ptr<_native::BindingError>> _native::ZatcaStageReport::finding(size_t index) const {
    auto result = _native::capi::fatoora_ZatcaStageReport_finding(this->AsFFI(),
        index);
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ZatcaFinding>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ZatcaFinding>>(std::unique_ptr<_native::ZatcaFinding>(_native::ZatcaFinding::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ZatcaFinding>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ZatcaStageReport* _native::ZatcaStageReport::AsFFI() const {
    return reinterpret_cast<const _native::capi::ZatcaStageReport*>(this);
}

inline _native::capi::ZatcaStageReport* _native::ZatcaStageReport::AsFFI() {
    return reinterpret_cast<_native::capi::ZatcaStageReport*>(this);
}

inline const _native::ZatcaStageReport* _native::ZatcaStageReport::FromFFI(const _native::capi::ZatcaStageReport* ptr) {
    return reinterpret_cast<const _native::ZatcaStageReport*>(ptr);
}

inline _native::ZatcaStageReport* _native::ZatcaStageReport::FromFFI(_native::capi::ZatcaStageReport* ptr) {
    return reinterpret_cast<_native::ZatcaStageReport*>(ptr);
}

inline void _native::ZatcaStageReport::operator delete(void* ptr) {
    _native::capi::fatoora_ZatcaStageReport_destroy(reinterpret_cast<_native::capi::ZatcaStageReport*>(ptr));
}


#endif // _NATIVE_ZatcaStageReport_HPP
