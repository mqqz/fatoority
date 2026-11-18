---
title: Migrate from fatoority 0.6 to 0.7
description: What changes between fatoority 0.6 and 0.7 in each language, from the removal of every name deprecated in 0.6 to the unified error classes, with a migration page per language.
---

# Migrate from 0.6 to 0.7

0.7 is a breaking release. Most of it removes the names that 0.6 kept as
deprecated aliases. Each one forwarded to a replacement that already exists in
0.6, so you can migrate on 0.6 first: build against 0.6 with deprecation
warnings turned into errors, replace every name the warnings point at, and then
upgrade. Each warning names its replacement.

The other changes have no deprecation step in 0.6. Make them while upgrading;
each language page lists them after the removed names.

| Language | Package | Migration page |
| --- | --- | --- |
| Rust | `fatoora-core = "0.7.0"` | [Rust](rust.md) |
| Python | `pip install "fatoority==0.7.0"` | [Python](python.md) |
| JavaScript / TypeScript | `fatoority@0.7.0` on npm | [JavaScript / TypeScript](javascript.md) |
| Kotlin / Java | `com.fatoority:fatoority:0.7.0` | [Kotlin / Java](kotlin.md) |
| C# / .NET | `Fatoority` 0.7.0 | [C# / .NET](csharp.md) |
| C / C++ | `fatoora.h` and the C++ headers from the 0.7.0 release assets | [C / C++](c.md) |

## Changes by language

A dash means the language is unaffected.

| Change | Rust | Python | JavaScript / TypeScript | Kotlin / Java | C# / .NET | C / C++ |
| --- | --- | --- | --- | --- | --- | --- |
| Names deprecated in 0.6 are removed | [Yes](rust.md#removed-items) | [Yes](python.md#removed-names) | [Yes](javascript.md#removed-names) | [Yes](kotlin.md#removed-members) | [Yes](csharp.md#removed-members) | - (none were deprecated) |
| One error hierarchy | - | [`FfiError` → `FatooraError`, `FfiErrorKind` → `ErrorKind`](python.md#error-classes) | [`ErrorCode` → `ErrorKind`, `TransportError` → `NetworkError`](javascript.md#error-classes) | [`BindingException` → `FatooraException`](kotlin.md#error-classes) | [`FfiException` → `FatooraException`, `FfiErrorKind` → `ErrorKind`](csharp.md#error-classes) | - |
| Client timeout | [Zero is `ZatcaError::InvalidTimeout`](rust.md#zero-request-timeout-is-invalid-input) | - (`ValueError`, as in 0.6) | - (no client timeout) | [`Duration` instead of `Long` ms](kotlin.md#client-timeout) | [`TimeSpan` instead of `uint` ms](csharp.md#client-timeout) | [Zero returns `FATOORA_INVALID_INPUT`](c.md#zero-timeout-is-invalid-input) |
| Clearance mode | - | [`clearance=ClearanceMode...` required](python.md#zatca-api-client) | [`ClearanceMode` required, parameter `clearanceMode`](javascript.md#clearance-mode-is-required-and-must-be-a-clearancemode) | [`Boolean` coroutine overloads removed](kotlin.md#boolean-coroutine-overloads) | - | - |
| `Text` handle removed; strings are plain or nullable | - | - | [`Text` export removed](javascript.md#text-class) | [`com.fatoora.Text` removed](kotlin.md#text-class) | [`Fatoora.Text` removed](csharp.md#text-class) | [C++ `fatoora::Text` removed](c.md#c-strings-replace-text) |
| Binary output | - | - | [`Bytes.asSlice()` → `Bytes.toUint8Array()`, `Uint8Array` inputs](javascript.md#binary-data-is-a-uint8array) | - | [`Bytes.AsSlice()` → `Bytes.ToArray()`](csharp.md#binary-output) | - |
| `InvoiceData` stores document-level allowances and charges only as entries | [Yes](rust.md#invoicedata-keeps-only-adjustment-entries) | - | - | - | - | - |
| Supported runtimes | - | [Python 3.10 or newer](python.md#python-310-or-newer) | - | - | - | - |
| New features | - | [`AsyncZatcaClient`](python.md#new-in-07) | - | - | - | - |

The CLI is unaffected. The bindings are now generated with Diplomat 0.17; that
changes only the generated low-level layers listed above.

If you are still on 0.5, follow
[Migrate from 0.5 to 0.6](../migrate-0.6/index.md) first, using the 0.6
replacement for every item marked deprecated there.

The full list of changes is in the
[changelog](https://github.com/mqqz/fatoority/blob/main/CHANGELOG.md).
