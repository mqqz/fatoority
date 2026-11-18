#ifndef fatoora_ZatcaStageReport_HPP
#define fatoora_ZatcaStageReport_HPP

#include "ZatcaStageReport.d.hpp"

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
#include "ZatcaFinding.hpp"
#include "ZatcaRuleSource.hpp"
#include "ZatcaStage.hpp"
#include "ZatcaStageStatus.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    fatoora::capi::ZatcaStage fatoora_ZatcaStageReport_stage(const fatoora::capi::ZatcaStageReport* self);

    fatoora::capi::ZatcaStageStatus fatoora_ZatcaStageReport_status(const fatoora::capi::ZatcaStageReport* self);

    typedef struct fatoora_ZatcaStageReport_provenance_result {union {fatoora::capi::ZatcaRuleSource* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_provenance_result;
    fatoora_ZatcaStageReport_provenance_result fatoora_ZatcaStageReport_provenance(const fatoora::capi::ZatcaStageReport* self);

    size_t fatoora_ZatcaStageReport_evaluated_assertions_len(const fatoora::capi::ZatcaStageReport* self);

    typedef struct fatoora_ZatcaStageReport_evaluated_assertion_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_evaluated_assertion_result;
    fatoora_ZatcaStageReport_evaluated_assertion_result fatoora_ZatcaStageReport_evaluated_assertion(const fatoora::capi::ZatcaStageReport* self, size_t index, diplomat::capi::DiplomatWrite* write);

    size_t fatoora_ZatcaStageReport_findings_len(const fatoora::capi::ZatcaStageReport* self);

    typedef struct fatoora_ZatcaStageReport_finding_result {union {fatoora::capi::ZatcaFinding* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_finding_result;
    fatoora_ZatcaStageReport_finding_result fatoora_ZatcaStageReport_finding(const fatoora::capi::ZatcaStageReport* self, size_t index);

    void fatoora_ZatcaStageReport_destroy(ZatcaStageReport* self);

    } // extern "C"
} // namespace capi
} // namespace

inline fatoora::ZatcaStage fatoora::ZatcaStageReport::stage() const {
    auto result = fatoora::capi::fatoora_ZatcaStageReport_stage(this->AsFFI());
    return fatoora::ZatcaStage::FromFFI(result);
}

inline fatoora::ZatcaStageStatus fatoora::ZatcaStageReport::status() const {
    auto result = fatoora::capi::fatoora_ZatcaStageReport_status(this->AsFFI());
    return fatoora::ZatcaStageStatus::FromFFI(result);
}

inline diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::ZatcaRuleSource>>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaStageReport::provenance() const {
    auto result = fatoora::capi::fatoora_ZatcaStageReport_provenance(this->AsFFI());
    return result.is_ok ? diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::ZatcaRuleSource>>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<diplomat::maybe_null<std::unique_ptr<fatoora::ZatcaRuleSource>>>(std::unique_ptr<fatoora::ZatcaRuleSource>(fatoora::ZatcaRuleSource::FromFFI(result.ok)))) : diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::ZatcaRuleSource>>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline size_t fatoora::ZatcaStageReport::evaluated_assertions_len() const {
    auto result = fatoora::capi::fatoora_ZatcaStageReport_evaluated_assertions_len(this->AsFFI());
    return result;
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaStageReport::evaluated_assertion(size_t index) const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaStageReport_evaluated_assertion(this->AsFFI(),
        index,
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaStageReport::evaluated_assertion_write(size_t index, W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaStageReport_evaluated_assertion(this->AsFFI(),
        index,
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline size_t fatoora::ZatcaStageReport::findings_len() const {
    auto result = fatoora::capi::fatoora_ZatcaStageReport_findings_len(this->AsFFI());
    return result;
}

inline diplomat::result<std::unique_ptr<fatoora::ZatcaFinding>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaStageReport::finding(size_t index) const {
    auto result = fatoora::capi::fatoora_ZatcaStageReport_finding(this->AsFFI(),
        index);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ZatcaFinding>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ZatcaFinding>>(std::unique_ptr<fatoora::ZatcaFinding>(fatoora::ZatcaFinding::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ZatcaFinding>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ZatcaStageReport* fatoora::ZatcaStageReport::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ZatcaStageReport*>(this);
}

inline fatoora::capi::ZatcaStageReport* fatoora::ZatcaStageReport::AsFFI() {
    return reinterpret_cast<fatoora::capi::ZatcaStageReport*>(this);
}

inline const fatoora::ZatcaStageReport* fatoora::ZatcaStageReport::FromFFI(const fatoora::capi::ZatcaStageReport* ptr) {
    return reinterpret_cast<const fatoora::ZatcaStageReport*>(ptr);
}

inline fatoora::ZatcaStageReport* fatoora::ZatcaStageReport::FromFFI(fatoora::capi::ZatcaStageReport* ptr) {
    return reinterpret_cast<fatoora::ZatcaStageReport*>(ptr);
}

inline void fatoora::ZatcaStageReport::operator delete(void* ptr) {
    fatoora::capi::fatoora_ZatcaStageReport_destroy(reinterpret_cast<fatoora::capi::ZatcaStageReport*>(ptr));
}


#endif // fatoora_ZatcaStageReport_HPP
