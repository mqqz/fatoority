---
title: ZATCA invoice QR payload reference
description: Read the Base64 TLV QR payload from a signed invoice and understand its fields, size limits and validation behavior.
---

# QR payloads

Signing generates a Base64-encoded TLV payload for the invoice. Pass this text to a QR image renderer to display or print it; the library returns text, not a PNG or SVG.

## Payload contract

| Value | Meaning |
| --- | --- |
| Tags 1–5 | Seller name, VAT registration number, timestamp, total including VAT and VAT total |
| Tags 6–8 | Invoice hash, signature and public key for stamped phase-two documents |
| Tag 9 | Certificate signature bytes; required for simplified invoices and their notes, prohibited on standard documents |
| Individual field size | At most 255 bytes per field; text uses UTF-8 and Arabic characters can occupy multiple bytes |
| Encoded size | At most 700 Base64 characters during generation |

`SignedInvoice::qr_code()` returns the exact Base64 text embedded in `xml()`. Signing and signed imports check QR structure, values and local signature integrity. Imports retain the supplied XML byte for byte. They reject mismatches instead of regenerating QR data.

Typed signing serializes the model first, then constructs QR fields from that XML. XML signing reads Tag 4 from `LegalMonetaryTotal/PayableAmount` (BT-115) and Tag 5 from the first document-level `TaxTotal/TaxAmount` in the document currency (BT-110). XML scalar values lose only surrounding XML whitespace (space, tab, CR, LF). Seller names retain their text. Decimal spellings such as `300`, `300.0` and `300.00` are preserved during construction. Both money fields use bounded exact XML-decimal comparison. The SDK validation profile additionally requires Tag 4's XML spelling to match, as observed in both the pinned SDK and development API. Signed import checks monetary integrity and retains the supplied payload; run full validation for formatting, amount scale and business rules.

Standard invoices receive their clearance stamp from ZATCA. A locally generated standard stamp is useful for SDK testing and submission preparation; it proves no clearance. Imports use the supplied document's signature and certificate, regardless of who created it. The supported standard stamp representation contains Tags 1–8; simplified stamps contain Tags 1–9. Unstamped CLI QR output contains only Tags 1–5 and is not a complete phase-two stamp.

The decoder enforces ascending, unique, nonempty tags, valid TLV framing, 255 bytes per field and 700 Base64 characters. Signature material must be valid Base64. Certificate signature metadata remains available on standard invoices even though Tag 9 is absent. Certificate issuer trust, revocation and gateway acceptance require separate evidence. The SDK validation profile continues to skip standard-invoice QR and signature checks. See [SDK compatibility](../development/sdk-parity.md).

## Read the payload

The getter takes no arguments and leaves the signed invoice usable.

=== "Rust"

    `SignedInvoice::qr_code(&self) -> &str` borrows the payload from the invoice. Copy with `.to_owned()` to keep it after dropping the invoice.

=== "Python"

    `SignedInvoice.qr_code() -> str` returns an independent string.

=== "JavaScript"

    `SignedInvoice.qrCode()` returns a string.

=== "C#"

    `SignedInvoice.QrCode()` returns a managed `string`.

=== "Kotlin"

    `SignedInvoice.qrCode(): String` returns a managed string.

=== "Java"

    `SignedInvoice.qrCode()` returns a `String`.

=== "C"

    `fatoora_signed_invoice_qr_code(invoice, &out, &error)` returns `FatooraStatus`. `invoice` is a live `const FatooraSignedInvoice *`; `out` is an owned `char *`. On success, release the payload with `fatoora_string_free(out)`. Release any error with `fatoora_error_free(error)`.

For a complete rendering workflow, see the [QR guide](../guides/qr.md).
