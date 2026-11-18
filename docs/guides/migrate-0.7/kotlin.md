---
title: Migrate a Kotlin or Java Project from fatoority 0.6 to 0.7
description: Every Kotlin and Java member removed from com.fatoora in 0.7, each with the 0.6 replacement to use instead.
---

# Migrate from 0.6 to 0.7: Kotlin and Java

This page covers the `com.fatoority:fatoority` and
`com.fatoority:fatoority-android` packages. The [overview](index.md) lists the
other languages.

=== "Gradle (Kotlin DSL)"

    ```kotlin
    dependencies {
        implementation("com.fatoority:fatoority:0.7.0")
        // Android: com.fatoority:fatoority-android:0.7.0
    }
    ```

=== "Maven"

    ```xml
    <dependency>
      <groupId>com.fatoority</groupId>
      <artifactId>fatoority</artifactId>
      <version>0.7.0</version>
    </dependency>
    ```

0.7 removes every `@Deprecated` member of the `com.fatoora` facade. Each
replacement below already exists in 0.6, so the simplest path is to compile
against 0.6 with warnings as errors (`allWarningsAsErrors = true` in Kotlin,
`-Xlint:deprecation -Werror` in Java), apply each `ReplaceWith` hint, and then
bump the version.

## Removed members

### Renamed aliases

| Removed | Use instead |
| --- | --- |
| `Adjustment.percentage(base, percent)` (factory) | `Adjustment.percentOf(base, percent)` |
| `adjustment.withSuppliedAmount(value)` | `adjustment.suppliedAmount(value)` |
| `adjustment.percent` property and `adjustment.percent()` | `adjustment.percentage` (`BigDecimal?`); Java `getPercentage()` |
| `allowanceCharge.withoutVatRate()` | `allowanceCharge.omitVatRate()` |
| `vatBreakdown.withoutVatRate()` | `vatBreakdown.omitVatRate()` |

### `String` amount overloads

Every method that takes a quantity, price, rate or amount keeps only its
`java.math.BigDecimal` overload. Wrap string literals in `BigDecimal("...")`
(Java: `new BigDecimal("...")`); never go through `Double`.

| Removed `String` overload | Use instead |
| --- | --- |
| `Adjustment.amount(value)` | `Adjustment.amount(BigDecimal(value))` |
| `Adjustment.percentOf(base, percent)`, `Adjustment.percentage(base, percent)` | `Adjustment.percentOf(BigDecimal(base), BigDecimal(percent))` |
| `adjustment.suppliedAmount(value)`, `adjustment.withSuppliedAmount(value)` | `adjustment.suppliedAmount(BigDecimal(value))` |
| `AllowanceCharge.create(kind, amount, reason, reasonCode, category, rate, exemption)` | The same call with `amount` and `rate` as `BigDecimal` |
| `allowanceCharge.withPercentage(base, percent)` | `withPercentage(BigDecimal(base), BigDecimal(percent))` |
| `InvoiceBuilder.addLineItem(description, quantity, unitCode, unitPrice, vatRate, category)` | The same call with `quantity`, `unitPrice` and `vatRate` as `BigDecimal` |
| `InvoiceBuilder.addAllowance(value, category, rate, exemption)`, `addCharge(...)`, `addTaxExemption(category, rate, exemption)` | The same calls with `rate` as `BigDecimal` |
| `InvoiceBuilder.setPrepaidAmount(amount)`, `setPayableRoundingAmount(amount)` | The same calls with a `BigDecimal` |
| `InvoiceBuilder.setTaxCurrency(currency, amount)` | `setTaxCurrency(currency, BigDecimal(amount))` |
| `InvoiceLineItem.create(description, quantity, unitCode, unitPrice, rate, category)` | The same call with `quantity`, `unitPrice` and `rate` as `BigDecimal` |
| `LineItemBuilder.setInvoicedQuantity`, `setItemNetPrice`, `setItemGrossPrice`, `setItemPriceDiscount`, `setItemPriceBaseQuantity`, `setLineNetAmount`, `setLineVatAmount`, `setLineAmountWithVat`, `setVatRate` (`String` argument) | The same setters with a `BigDecimal` |
| `VatBreakdown.create(category, rate, taxableAmount, taxAmount, exemption)` | The same call with `rate`, `taxableAmount` and `taxAmount` as `BigDecimal` |

### `Boolean` coroutine overloads

| Removed | Use instead |
| --- | --- |
| `client.reportSimplifiedInvoiceAsync(invoice, credentials, clearanceStatus: Boolean, acceptLanguage)` | `client.reportSimplifiedInvoiceAsync(invoice, credentials, ClearanceMode.Enabled` or `ClearanceMode.Disabled, acceptLanguage)` |
| `client.clearStandardInvoiceAsync(invoice, credentials, clearanceStatus: Boolean, acceptLanguage)` | `client.clearStandardInvoiceAsync(invoice, credentials, ClearanceMode.Enabled` or `ClearanceMode.Disabled, acceptLanguage)` |

`true` maps to `ClearanceMode.Enabled` and `false` to `ClearanceMode.Disabled`,
as in the blocking methods.

### Getter functions

The 0.5-style getter functions are gone; read the property of the same name
instead. In Kotlin drop the parentheses (`address.city()` becomes
`address.city`); in Java call the generated getter (`address.getCity()`).
`is*` and `has*` getters had no deprecated function and are unchanged.

The property types differ from the removed functions in two ways:

- Getters marked "was `Text?`" returned a closeable `Text` handle. The property
  is a nullable `String`, or a nullable `BigDecimal` for amounts
  (`baseAmount`, `percentage`, `itemGrossPrice`, `itemPriceDiscount`,
  `itemPriceBaseQuantity`, `taxCurrencyAmount`). Drop the `.use { it.value() }`.
- Getters marked "was `String`" returned decimal text. The property is a
  `BigDecimal`; call `toPlainString()` where you need the text, and compare with
  `compareTo`, since returned values are normalized (`10.50` reads back as
  `10.5`).

| Class | Removed getter functions |
| --- | --- |
| `Address` | `additionalNumber()` (was `Text?`), `additionalStreet()` (was `Text?`), `buildingNumber()`, `city()`, `countryCode()`, `district()` (was `Text?`), `postalCode()`, `province()`, `street()` |
| `Adjustment` | `baseAmount()` (was `Text?`), `reason()` (was `Text?`), `reasonCode()` (was `Text?`), `value()` (was `String`) |
| `AllowanceCharge` | `amount()` (was `String`), `baseAmount()` (was `Text?`), `category()`, `kind()`, `percentage()` (was `Text?`), `rate()` (was `String`), `reason()` (was `Text?`), `reasonCode()` (was `Text?`), `taxCategoryCount()` |
| `ComplianceCsid` | `binarySecurityToken()`, `env()`, `requestId()` (was `Text?`), `secret()` |
| `ComplianceRequest` | `headersJson()`, `method()`, `url()` |
| `Config` | `env()` |
| `Csr` | `subjectString()` |
| `FinalizedInvoice` | `hash()`, `xml()` |
| `InvoiceData` | `actualDeliveryDate()` (was `Text?`), `adjustmentsLen()`, `allowanceReason()` (was `Text?`), `billingReferencesLen()`, `contractId()` (was `Text?`), `currency()`, `flags()`, `id()`, `invoiceCounter()`, `invoiceLevelCharge()` (was `String`), `invoiceLevelDiscount()` (was `String`), `invoiceSubType()`, `invoiceTypeKind()`, `issueDate()`, `issueDatetime()`, `issueTime()`, `latestDeliveryDate()` (was `Text?`), `lineItemsLen()`, `notesLen()`, `originalInvoiceReason()` (was `Text?`), `paymentInstruction()` (was `Text?`), `paymentMeansCode()`, `paymentMeansLen()`, `previousInvoiceHash()`, `purchaseOrderId()` (was `Text?`), `taxCurrency()` (was `Text?`), `taxCurrencyAmount()` (was `Text?`), `uuid()`, `vatCategory()` |
| `InvoiceFlag` | `bit()` |
| `InvoiceLineItem` | `adjustmentsLen()`, `description()`, `documentReferencesLen()`, `id()` (was `Text?`), `itemGrossPrice()` (was `Text?`), `itemPriceBaseQuantity()` (was `Text?`), `itemPriceBaseQuantityUnitCode()` (was `Text?`), `itemPriceDiscount()` (was `Text?`), `quantity()` (was `String`), `taxSubtotalsLen()`, `totalAmount()` (was `String`), `unitCode()`, `unitPrice()` (was `String`), `vatAmount()` (was `String`), `vatCategory()`, `vatRate()` (was `String`) |
| `InvoiceNote` | `language()`, `languageId()` (was `Text?`), `text()` |
| `InvoiceRequest` | `headersJson()`, `method()`, `url()` |
| `InvoiceSigner` | `certificatePem()` |
| `InvoiceTotals` | `allowanceTotal()` (was `String`), `chargeTotal()` (was `String`), `lineExtension()` (was `String`), `payableAmount()` (was `String`), `payableRoundingAmount()` (was `String`), `prepaidAmount()` (was `String`), `taxAmount()` (was `String`), `taxInclusive()` (was `String`), `taxableAmount()` (was `String`), `vatBreakdownLen()` |
| `OriginalInvoiceRef` | `documentTypeCode()` (was `Text?`), `id()`, `issueDate()` (was `Text?`), `issueTime()` (was `Text?`), `uuid()` (was `Text?`) |
| `OtherId` | `scheme()` (was `Text?`), `value()` |
| `Party` | `identifiersLen()`, `name()` |
| `PaymentMeans` | `accountIdentifier()` (was `Text?`), `code()`, `instructionNote()` (was `Text?`), `instructionNotesLen()`, `paymentTerms()` (was `Text?`) |
| `ProductionCsid` | `binarySecurityToken()`, `env()`, `requestId()` (was `Text?`), `secret()` |
| `ProductionRequest` | `headersJson()`, `method()`, `url()` |
| `SignedInvoice` | `certHash()`, `hash()`, `invoiceHash()`, `issuer()`, `publicKey()`, `qrCode()`, `serial()`, `signature()`, `signedPropsHash()`, `signingTime()`, `xml()`, `zatcaKeySignature()` (was `Text?`) |
| `TaxExemption` | `reason()` (was `Text?`), `reasonCode()` (was `Text?`) |
| `Text` | `value()` |
| `ValidationMessage` | `category()` (was `Text?`), `code()` (was `Text?`), `message()` (was `Text?`), `messageType()`, `messageTypeRaw()` (was `Text?`), `status()`, `statusRaw()` (was `Text?`) |
| `ValidationResponse` | `clearanceStatus()`, `clearanceStatusRaw()` (was `Text?`), `clearedInvoiceBase64()` (was `Text?`), `clearedInvoiceXml()` (was `Text?`), `httpStatus()`, `outcome()`, `qrBuyerStatus()` (was `Text?`), `qrSellerStatus()` (was `Text?`), `reportingStatus()`, `reportingStatusRaw()` (was `Text?`) |
| `ValidationResults` | `errorLen()`, `infoLen()`, `status()`, `statusRaw()` (was `Text?`), `warningLen()` |
| `VatBreakdown` | `category()`, `rate()` (was `String`), `taxAmount()` (was `String`), `taxableAmount()` (was `String`) |
| `VatId` | `value()` |
| `ZatcaFinding` | `assertionSite()`, `code()`, `layer()`, `locationColumn()`, `locationKind()`, `locationLine()`, `locationPath()` (was `Text?`), `message()`, `severity()` |
| `ZatcaRuleSource` | `sha256()`, `stylesheet()` |
| `ZatcaStageReport` | `evaluatedAssertionsLen()`, `findingsLen()`, `stage()`, `status()` |
| `ZatcaValidationReport` | `evaluatedAt()`, `profile()`, `schemaVersion()`, `stagesLen()` |

The enum function `InvoiceFlag.bit()` follows the same rule: use
`InvoiceFlag.Export.bit` in Kotlin and `InvoiceFlag.Export.getBit()` in Java.

### `Text` class

`com.fatoora.Text` is removed. Only the removed getter functions returned it;
every optional text property is already a `String?` in 0.6, so replace
`text.value` or `text.value()` with the string itself and drop the
`use { }` block.

## Example

```kotlin
// 0.6, with deprecation warnings
val city: String = address.city()
val district: String? = address.district()?.use { it.value() }
val payable: String = invoice.totals().use { it.payableAmount() }
Adjustment.percentage(BigDecimal("200"), BigDecimal("10")).use { /* ... */ }
builder.addLineItem("Notebook", "2", "PCE", "50", "15", VatCategory.Standard)
client.reportSimplifiedInvoiceAsync(signed, credentials, false)
```

```kotlin
// 0.7
val city: String = address.city
val district: String? = address.district
val payable: BigDecimal = invoice.totals().use { it.payableAmount }
Adjustment.percentOf(BigDecimal("200"), BigDecimal("10")).use { /* ... */ }
builder.addLineItem("Notebook", BigDecimal("2"), "PCE", BigDecimal("50"), BigDecimal("15"), VatCategory.Standard)
client.reportSimplifiedInvoiceAsync(signed, credentials, ClearanceMode.Disabled)
```

```java
// 0.7 (Java)
String city = address.getCity();
String district = address.getDistrict();
BigDecimal payable;
try (InvoiceTotals totals = invoice.totals()) {
    payable = totals.getPayableAmount();
}
```
