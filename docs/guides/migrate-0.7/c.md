---
title: Migrate C and C++ Code from fatoority 0.6 to 0.7
description: Upgrade C and C++ code from the fatoority 0.6 native library to 0.7. The C facade keeps its signatures; the C++ headers drop the Text handle and return optional strings.
---

# Migrate from 0.6 to 0.7: C and C++

This page covers the C facade (`fatoora.h`), the generated C headers and the
C++ headers. The [overview](index.md) lists the other languages.

Take `fatoora.h`, `fatoora.c`, the generated headers and the shared library
from the same 0.7.0
[release assets](https://github.com/mqqz/fatoority/releases); mixing 0.6
headers with a 0.7 library, or the reverse, is not supported. 0.6 deprecated
nothing in C or C++, so there are no removed aliases.

## C facade

Every `fatoora.h` function keeps its 0.6 signature and behavior, with one
exception.

### Zero timeout is invalid input

`fatoora_zatca_client_with_timeout(config, 0, &client, &error)` returns
`FATOORA_INVALID_INPUT` (details type `invalid_timeout`) instead of
`FATOORA_INTERNAL`. Use `fatoora_zatca_client_new` for no client-side timeout.

## C++ strings replace `Text`

The `fatoora::Text` class and its header are removed. Every getter that
returned a `Text` returns a string:

- Optional text returns `std::optional<std::string>` and no longer fails:
  `std::nullopt` means absent. This covers getters such as
  `Address::district()`, `Adjustment::reason()`, `ComplianceCsid::request_id()`
  and the `ValidationResponse` `*_raw()` statuses.
- `ValidationResponse::cleared_invoice_xml()` and
  `SignedInvoice::zatca_key_signature()` return
  `diplomat::result<std::string, ...>` and fail when the value is absent. Check
  `cleared_invoice_xml_present()` or `zatca_key_signature_present()` first.

```diff
-std::unique_ptr<fatoora::Text> district = address->district().ok().value();
-if (district) {
-    std::string text = district->value().ok().value();
-}
+std::optional<std::string> district = address->district();
+if (district) {
+    std::string text = *district;
+}
```

```diff
-std::unique_ptr<fatoora::Text> xml = response->cleared_invoice_xml().ok().value();
-if (xml) {
-    save(xml->value().ok().value());
-}
+if (response->cleared_invoice_xml_present()) {
+    save(response->cleared_invoice_xml().ok().value());
+}
```

A `_write(W&)` overload of each getter writes into your own buffer.

## Generated C headers

Code that calls the generated `bindings/c/generated` functions directly, rather
than `fatoora.h`, sees the same change: `Text.h` is removed, and each optional
text getter takes a `DiplomatWrite *` and returns a result whose `is_ok` is
`false` when the value is absent. The facade handles this for you.
