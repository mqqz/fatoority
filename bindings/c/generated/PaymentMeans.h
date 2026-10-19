#ifndef PaymentMeans_H
#define PaymentMeans_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Text.d.h"

#include "PaymentMeans.d.h"






typedef struct fatoora_PaymentMeans_with_instruction_note_result {union {PaymentMeans* ok; BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_with_instruction_note_result;
fatoora_PaymentMeans_with_instruction_note_result fatoora_PaymentMeans_with_instruction_note(const PaymentMeans* self, DiplomatStringView value);

size_t fatoora_PaymentMeans_instruction_notes_len(const PaymentMeans* self);

typedef struct fatoora_PaymentMeans_instruction_note_at_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_instruction_note_at_result;
fatoora_PaymentMeans_instruction_note_at_result fatoora_PaymentMeans_instruction_note_at(const PaymentMeans* self, size_t index);

typedef struct fatoora_PaymentMeans_new_result {union {PaymentMeans* ok; BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_new_result;
fatoora_PaymentMeans_new_result fatoora_PaymentMeans_new(DiplomatStringView code, OptionStringView instruction);

typedef struct fatoora_PaymentMeans_with_account_result {union {PaymentMeans* ok; BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_with_account_result;
fatoora_PaymentMeans_with_account_result fatoora_PaymentMeans_with_account(const PaymentMeans* self, OptionStringView id, OptionStringView terms);

typedef struct fatoora_PaymentMeans_code_result {union { BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_code_result;
fatoora_PaymentMeans_code_result fatoora_PaymentMeans_code(const PaymentMeans* self, DiplomatWrite* write);

typedef struct fatoora_PaymentMeans_instruction_note_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_instruction_note_result;
fatoora_PaymentMeans_instruction_note_result fatoora_PaymentMeans_instruction_note(const PaymentMeans* self);

bool fatoora_PaymentMeans_has_account(const PaymentMeans* self);

typedef struct fatoora_PaymentMeans_account_identifier_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_account_identifier_result;
fatoora_PaymentMeans_account_identifier_result fatoora_PaymentMeans_account_identifier(const PaymentMeans* self);

typedef struct fatoora_PaymentMeans_payment_terms_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_payment_terms_result;
fatoora_PaymentMeans_payment_terms_result fatoora_PaymentMeans_payment_terms(const PaymentMeans* self);

void fatoora_PaymentMeans_destroy(PaymentMeans* self);





#endif // PaymentMeans_H
