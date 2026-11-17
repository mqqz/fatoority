---
title: Migrate a Kotlin or Java Project from fatoority 0.5 to 0.6
description: Upgrade Kotlin and Java code from fatoority 0.5 to 0.6. Enums replace integer codes, getters become properties, amounts become BigDecimal and failures throw typed exceptions.
---

# Migrate from 0.5 to 0.6: Kotlin and Java

The `com.fatoora` API changes in four ways in 0.6: integer codes and Booleans
become enums, class and method names follow the Rust core, value getters
become Kotlin properties with `BigDecimal` amounts, and native failures throw
one `BindingException` subclass per error code. The 0.5 getter functions,
`String` amount overloads and `Boolean` coroutine overloads remain as
`@Deprecated` members with `ReplaceWith` hints until 0.7. Renamed classes and
methods do not, so the compiler reports each one.

=== "Gradle (Kotlin DSL)"

    ```kotlin
    dependencies {
        implementation("com.fatoority:fatoority:0.6.0")
        // Android: com.fatoority:fatoority-android:0.6.0
    }
    ```

=== "Maven"

    ```xml
    <dependency>
      <groupId>com.fatoority</groupId>
      <artifactId>fatoority</artifactId>
      <version>0.6.0</version>
    </dependency>
    ```

The JAR and AAR bundle their native libraries.

## Upgrade checklist

1. Replace integer codes and Booleans with enums: `Environment.NonProduction`
   for `0`, `InvoiceTypeKind.Tax` and `InvoiceSubType.Simplified` for `0, 1`,
   `VatCategory.Standard` for `1`, `ClearanceMode.Disabled` for `false`.
   The enum discriminants equal the old codes.
2. Rename classes and methods (see the [table below](#renamed-names)):
   `Signer` to `InvoiceSigner`, `CsidCompliance`/`CsidProduction` to
   `ComplianceCsid`/`ProductionCsid`, `postCsrForCcsid`/`postCcsidForPcsid`
   to `requestComplianceCsid`/`requestProductionCsid` (and their `*Async`
   helpers), `fromPropertiesStr` to `fromStr`, `hashBase64()` to `hash`.
3. Drop the `Config` argument from `Xml.validate` and `Xml.validateZatca`, and
   read the returned `ZatcaValidationReport` instead of parsing JSON.
4. Build once and work through the deprecation warnings. In Kotlin,
   `address.city()` becomes `address.city`; in Java it becomes
   `address.getCity()`. Optional text is a `String?` instead of a closeable
   `Text`. Pass `BigDecimal` amounts instead of strings.
5. Kotlin only: `isValid()`, `isComplete()`, `hasErrors()`, `hasVatRate()` and
   the other `is*`/`has*` getters are properties with no deprecated function
   (it would clash with the Java getter), so drop the parentheses. Java callers
   keep `report.isValid()`.

## Build an invoice

=== "0.5"

    ```kotlin
    import com.fatoora.*

    InvoiceBuilder.create(0, 1).use { builder ->
        builder.setId("INV-1001")
        builder.setUuid(uuid)
        builder.setIssueDatetime("2026-10-26T09:00:00Z")
        builder.setCurrency("SAR")
        builder.setInvoiceCounter(java.math.BigInteger.ONE)
        builder.setPreviousInvoiceHash(previousHash)
        builder.setPaymentMeansCode("10")
        builder.setVatCategory(1)
        Address.create("SA", "Riyadh", "King Fahd Road", "1234", "12222", null, "5678", "Olaya").use { address ->
            builder.setSeller("Example Store", address, "399999999900003", "7003339333", "CRN")
        }
        builder.addLineItem("Notebook", "2", "PCE", "50", "15", 1)
        builder.invoiceLevelDiscount("10")
        builder.allowanceReason("Loyalty discount")
        builder.build().use { invoice ->
            val payable: String = invoice.totals().use { it.payableAmount() }
            println(payable)
        }
    }
    ```

=== "0.6"

    ```kotlin
    import com.fatoora.*
    import java.math.BigDecimal

    InvoiceBuilder.create(InvoiceTypeKind.Tax, InvoiceSubType.Simplified).use { builder ->
        builder.setId("INV-1001")
        builder.setUuid(uuid)
        builder.setIssueDatetime("2026-10-26T09:00:00Z")
        builder.setCurrency("SAR")
        builder.setInvoiceCounter(java.math.BigInteger.ONE)
        builder.setPreviousInvoiceHash(previousHash)
        PaymentMeans.create("10").use { builder.addPaymentMeans(it) }
        builder.setVatCategory(VatCategory.Standard)
        Address.create("SA", "Riyadh", "King Fahd Road", "1234", "12222", null, "5678", "Olaya").use { address ->
            builder.setSeller("Example Store", address, "399999999900003", "7003339333", "CRN")
        }
        builder.addLineItem("Notebook", BigDecimal("2"), "PCE", BigDecimal("50"), BigDecimal("15"), VatCategory.Standard)
        Adjustment.amount(BigDecimal("10")).use { amount ->
            amount.withReason("Loyalty discount").use { discount ->
                TaxExemption.create().use { exemption ->
                    builder.addAllowance(discount, VatCategory.Standard, BigDecimal("15"), exemption)
                }
            }
        }
        builder.build().use { invoice ->
            val payable: BigDecimal = invoice.totals().use { it.payableAmount }
            println(payable)
        }
    }
    ```

`setPaymentMeansCode`, `setAllowance`, `allowanceReason`, `invoiceLevelCharge`
and `invoiceLevelDiscount` are gone from the builder. `addAllowance` and
`addCharge` append one entry each in the VAT category and at the rate you pass.
Returned amounts are normalized (`10.50` reads back as `10.5`); compare
`BigDecimal`s with `compareTo`, not `equals`.

## Sign, validate and report

=== "0.5"

    ```kotlin
    Signer.fromPem(certificatePem, keyPem).use { signer ->
        signer.sign(invoice).use { signed ->
            Config.create(0).use { config ->
                val options = """{"previous_invoice_hash":"$previousHash"}"""
                val json = Xml.validateZatca(config, signed.xml(), options)
                // Parse is_valid from json with a JSON library.
                ZatcaClient.create(config).use { client ->
                    CsidProduction.create(0, null, token, secret).use { credentials ->
                        client.reportSimplifiedInvoice(signed, credentials, false, "en").use { response ->
                            val status: String? = response.reportingStatus()?.use { it.value() }
                            response.ensureAccepted()
                        }
                    }
                }
            }
        }
    }
    ```

=== "0.6"

    ```kotlin
    InvoiceSigner.fromPem(certificatePem, keyPem).use { signer ->
        signer.sign(invoice).use { signed ->
            val options = """{"previous_invoice_hash":"$previousHash"}"""
            Xml.validateZatca(signed.xml, options).use { report ->
                check(report.isValid) { report.toJson() }
            }
            Config.create(Environment.NonProduction).use { config ->
                ZatcaClient.create(config).use { client ->
                    ProductionCsid.create(Environment.NonProduction, null, token, secret).use { credentials ->
                        client.reportSimplifiedInvoice(signed, credentials, ClearanceMode.Disabled, "en").use { response ->
                            val status: ReportingStatus? = response.reportingStatus
                            val raw: String? = response.reportingStatusRaw
                            response.ensureAccepted()
                        }
                    }
                }
            }
        }
    }
    ```

To bound each request, create the client with
`ZatcaClient.createWithTimeout(config, 30_000)` (milliseconds); a timeout
throws `NetworkException`. The coroutine helpers are renamed with the blocking
methods (`requestComplianceCsidAsync`, `requestProductionCsidAsync`), and
`reportSimplifiedInvoiceAsync`/`clearStandardInvoiceAsync` take a
`ClearanceMode`; their `Boolean` overloads are deprecated.

The same flow in Java:

=== "0.5"

    ```java
    try (Config config = Config.create(0);
         ZatcaClient client = ZatcaClient.create(config);
         CsidProduction credentials = CsidProduction.create(0, null, token, secret);
         SignedInvoice invoice = SignedInvoice.fromXml(signedXml);
         ValidationResponse response = client.reportSimplifiedInvoice(invoice, credentials, false, "en")) {
        String json = Xml.validateZatca(config, signedXml, null);
        System.out.println(invoice.qrCode());
        response.ensureAccepted();
    } catch (BindingException error) {
        System.err.println(error.getCode() + ": " + error.getDetailsJson());
    }
    ```

=== "0.6"

    ```java
    try (Config config = Config.create(Environment.NonProduction);
         ZatcaClient client = ZatcaClient.create(config);
         ProductionCsid credentials = ProductionCsid.create(Environment.NonProduction, null, token, secret);
         SignedInvoice invoice = SignedInvoice.fromXml(signedXml);
         ZatcaValidationReport report = Xml.validateZatca(signedXml);
         ValidationResponse response = client.reportSimplifiedInvoice(invoice, credentials, ClearanceMode.Disabled, "en")) {
        System.out.println(report.isValid() + " " + invoice.getQrCode());
        response.ensureAccepted();
    } catch (ApiException error) {
        System.err.println("Rejected: " + error.getDetailsJson());
    } catch (BindingException error) {
        System.err.println(error.getKind() + " (" + error.getCode() + "): " + error.getDetailsJson());
    }
    ```

## Errors

`BindingException` is now `open` and gains `kind: ErrorKind?`. Native failures
throw the subclass for their code: `InvalidInputException`,
`ValidationException`, `ParseException`, `XmlException`, `CryptoException`,
`IoException`, `NetworkException`, `UnauthorizedException`,
`InternalException` or `ApiException`. Existing `catch (BindingException)`
blocks keep catching everything; add narrower `catch` clauses before it where
you handle a specific failure.

## Renamed names

None of these have a deprecated alias.

| 0.5 | 0.6 |
| --- | --- |
| `Config.create(0)` and other `Int` environment, invoice kind, subtype and VAT category arguments | `Environment`, `InvoiceTypeKind`, `InvoiceSubType`, `VatCategory` enums |
| `Boolean` charge argument of `AllowanceCharge.create` | `AdjustmentKind.Allowance`/`Charge` |
| `Boolean clearanceStatus` in `reportSimplifiedInvoice`, `clearStandardInvoice` and `prepare*` | `ClearanceMode.Disabled`/`Enabled` |
| `CsidCompliance`, `CsidProduction` | `ComplianceCsid`, `ProductionCsid` |
| `client.postCsrForCcsid(csr, otp)`, `postCcsidForPcsid(ccsid)` (and `prepare*`, `*Async`) | `client.requestComplianceCsid(csr, otp)`, `requestProductionCsid(ccsid)` |
| `Signer` | `InvoiceSigner` |
| `CsrProperties.fromPropertiesStr(text)`, `parseCsrConfigFile(path)` | `CsrProperties.fromStr(text)`, `fromFile(path)` |
| `invoice.hashBase64()` | `invoice.hash` (`signed.invoiceHash` is the stored digest) |
| `builder.flags(mask)`, `data.flagsRaw()` | `builder.setFlags(mask)`, `data.flags` |
| `builder.setTaxExemption(category, rate, exemption)` | `builder.addTaxExemption(category, rate, exemption)` |
| `builder.addDocumentAdjustment(charge, adjustment, category, rate, exemption)` | `builder.addAllowance(adjustment, category, rate, exemption)` or `addCharge(...)` |
| `builder.setNote(language, text)`, `setNoteWithoutLanguage(text)` | `builder.addNote(language, text)`, with `null` for no language |
| `builder.setPaymentMeansCode(code)` | `builder.addPaymentMeans(PaymentMeans.create(code))` |
| `builder.setAllowance(reason, amount)`, `allowanceReason`, `invoiceLevelCharge`, `invoiceLevelDiscount` | `addAllowance`/`addCharge` with `Adjustment.amount(amount).withReason(reason)` |
| `charge.isCharge()`, `item.adjustmentIsCharge(i)` | `charge.kind`, `item.adjustmentKind(i)` returning `AdjustmentKind` |
| `response.reportingStatus()` and the other status getters returning `Text?` | `ReportingStatus?`, `ClearanceStatus?`, `ValidationStatus?`, `MessageType?`; `*Raw` properties hold the text |
| `Xml.validate(config, xml)` | `Xml.validate(xml)` |
| `Xml.validateZatca(config, xml, optionsJson)` returning JSON text | `Xml.validateZatca(xml, optionsJson)` returning `ZatcaValidationReport`; `toJson()` gives the old text |
| `PaymentMeans.instructionNoteAt(i)`, `ZatcaStageReport.evaluatedAssertion(i)` returning `Text` | Return `String` |

Credential `requestId` is `null` when the credentials have no request ID.
