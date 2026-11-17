#ifndef fatoora_ZatcaValidationReport_HPP
#define fatoora_ZatcaValidationReport_HPP

#include "ZatcaValidationReport.d.hpp"

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
#include "ZatcaStageReport.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    uint32_t fatoora_ZatcaValidationReport_schema_version(const fatoora::capi::ZatcaValidationReport* self);

    typedef struct fatoora_ZatcaValidationReport_profile_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_profile_result;
    fatoora_ZatcaValidationReport_profile_result fatoora_ZatcaValidationReport_profile(const fatoora::capi::ZatcaValidationReport* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ZatcaValidationReport_evaluated_at_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_evaluated_at_result;
    fatoora_ZatcaValidationReport_evaluated_at_result fatoora_ZatcaValidationReport_evaluated_at(const fatoora::capi::ZatcaValidationReport* self, diplomat::capi::DiplomatWrite* write);

    bool fatoora_ZatcaValidationReport_is_complete(const fatoora::capi::ZatcaValidationReport* self);

    bool fatoora_ZatcaValidationReport_has_errors(const fatoora::capi::ZatcaValidationReport* self);

    bool fatoora_ZatcaValidationReport_is_valid(const fatoora::capi::ZatcaValidationReport* self);

    size_t fatoora_ZatcaValidationReport_stages_len(const fatoora::capi::ZatcaValidationReport* self);

    typedef struct fatoora_ZatcaValidationReport_stage_result {union {fatoora::capi::ZatcaStageReport* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_stage_result;
    fatoora_ZatcaValidationReport_stage_result fatoora_ZatcaValidationReport_stage(const fatoora::capi::ZatcaValidationReport* self, size_t index);

    typedef struct fatoora_ZatcaValidationReport_to_json_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_to_json_result;
    fatoora_ZatcaValidationReport_to_json_result fatoora_ZatcaValidationReport_to_json(const fatoora::capi::ZatcaValidationReport* self, diplomat::capi::DiplomatWrite* write);

    void fatoora_ZatcaValidationReport_destroy(ZatcaValidationReport* self);

    } // extern "C"
} // namespace capi
} // namespace

inline uint32_t fatoora::ZatcaValidationReport::schema_version() const {
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_schema_version(this->AsFFI());
    return result;
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaValidationReport::profile() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_profile(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaValidationReport::profile_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_profile(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaValidationReport::evaluated_at() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_evaluated_at(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaValidationReport::evaluated_at_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_evaluated_at(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline bool fatoora::ZatcaValidationReport::is_complete() const {
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_is_complete(this->AsFFI());
    return result;
}

inline bool fatoora::ZatcaValidationReport::has_errors() const {
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_has_errors(this->AsFFI());
    return result;
}

inline bool fatoora::ZatcaValidationReport::is_valid() const {
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_is_valid(this->AsFFI());
    return result;
}

inline size_t fatoora::ZatcaValidationReport::stages_len() const {
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_stages_len(this->AsFFI());
    return result;
}

inline diplomat::result<std::unique_ptr<fatoora::ZatcaStageReport>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaValidationReport::stage(size_t index) const {
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_stage(this->AsFFI(),
        index);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ZatcaStageReport>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ZatcaStageReport>>(std::unique_ptr<fatoora::ZatcaStageReport>(fatoora::ZatcaStageReport::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ZatcaStageReport>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaValidationReport::to_json() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_to_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaValidationReport::to_json_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaValidationReport_to_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ZatcaValidationReport* fatoora::ZatcaValidationReport::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ZatcaValidationReport*>(this);
}

inline fatoora::capi::ZatcaValidationReport* fatoora::ZatcaValidationReport::AsFFI() {
    return reinterpret_cast<fatoora::capi::ZatcaValidationReport*>(this);
}

inline const fatoora::ZatcaValidationReport* fatoora::ZatcaValidationReport::FromFFI(const fatoora::capi::ZatcaValidationReport* ptr) {
    return reinterpret_cast<const fatoora::ZatcaValidationReport*>(ptr);
}

inline fatoora::ZatcaValidationReport* fatoora::ZatcaValidationReport::FromFFI(fatoora::capi::ZatcaValidationReport* ptr) {
    return reinterpret_cast<fatoora::ZatcaValidationReport*>(ptr);
}

inline void fatoora::ZatcaValidationReport::operator delete(void* ptr) {
    fatoora::capi::fatoora_ZatcaValidationReport_destroy(reinterpret_cast<fatoora::capi::ZatcaValidationReport*>(ptr));
}


#endif // fatoora_ZatcaValidationReport_HPP
