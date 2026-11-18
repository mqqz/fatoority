---
title: Kotlin, Java and Android ZATCA E-Invoicing Library
description: Install fatoority from Maven Central for Kotlin, Java, or Android. Find platform requirements and ZATCA integration examples.
---

# Kotlin and Java package reference

Use `com.fatoora` classes to build, sign, and validate invoices and send ZATCA API
requests on the JVM or Android. Value getters are Kotlin properties (Java
`getX()`), amounts are exact `BigDecimal` values, and failures throw a
`FatooraException` subclass for each error kind.

## Install from Maven Central

Use a published release from Maven Central. Java/Kotlin imports remain
`com.fatoora`; the Maven group is `com.fatoority`.

For desktop Gradle projects, add this to `build.gradle.kts`:

```kotlin
repositories { mavenCentral() }
dependencies {
    implementation("com.fatoority:fatoority:0.7.0")
}
```

For Android, include `mavenCentral()` in the repositories under
`dependencyResolutionManagement` in `settings.gradle.kts`, then add the Android
artifact to your app's `build.gradle.kts`:

```kotlin
dependencies {
    implementation("com.fatoority:fatoority-android:0.7.0")
}
```

Choose the desktop JAR or Android AAR, not both. They bundle the native libraries
for their supported platforms and resolve their runtime dependencies transitively.
Consumers do not need Rust or a separate native-library build.

| Artifact | Runtime | Bundled native platforms |
| --- | --- | --- |
| `com.fatoority:fatoority:0.7.0` | Java 8+ | Linux x86-64, Windows x86-64, macOS arm64 |
| `com.fatoority:fatoority-android:0.7.0` | Android API 26+ | arm64-v8a, x86_64 |

Linux requires glibc 2.28+; musl/Alpine is not included. Desktop binaries cannot
be used on Android; use the Android artifact for Android applications.

Windows x86-64 also requires the [Microsoft Visual C++ v14 x64 runtime](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist).
The native DLL imports `VCRUNTIME140.dll`; it is not bundled in the JAR.

## Kotlin and Java

Select your language for resource handling and examples.

=== "Kotlin"

    ```kotlin
    import com.fatoora.*

    SigningKey.generate().use { key ->
        val privateKeyPem = key.toPem()
        // Store privateKeyPem securely.
    }
    ```

    Use `use` for native owners, including objects returned from methods.
    Getters that return copied values are properties, such as `address.city`
    (`String`) or `address.district` (`String?`).

=== "Java"

    ```java
    import com.fatoora.*;

    try (SigningKey key = SigningKey.generate()) {
        String privateKeyPem = key.toPem();
        // Store privateKeyPem securely.
    }
    ```

    Use try-with-resources for native owners, including returned objects.
    Kotlin properties appear as getters, such as `address.getCity()`; boolean
    properties keep their names, such as `report.isValid()` and `report.hasErrors()`.
    Optional text getters return a nullable `String`.

Use `Environment.NonProduction`, `Environment.Simulation`, or
`Environment.Production`. Configuration, CSR and credentials must agree on the
environment. Factory names use `create`, such as
`Config.create(Environment.NonProduction)`; other method names use camel case.

## Ownership and values

| Value | Public JVM value |
| --- | --- |
| Native object | `AutoCloseable`; close after use |
| Value getter (text, amount, number, enum, boolean) | Kotlin `val` property; Java `getX()`, `isX()` or `hasX()` |
| Getter returning a native object | Method, such as `invoice.totals()`; close the result |
| Optional text input | Nullable `String`; `null` differs from `""` |
| Optional text output | Nullable `String` |
| Quantity, price, rate or amount | `java.math.BigDecimal`, converted exactly in both directions |
| Unsigned byte / short / int | Checked JVM integer / integer / long |
| Enumerated code | `com.fatoora` enum with the native variant names: `Environment`, `InvoiceTypeKind`, `InvoiceSubType`, `VatCategory`, `AdjustmentKind`, `ClearanceMode`, `InvoiceOutcome` |
| Response status | Nullable `ReportingStatus`, `ClearanceStatus`, `ValidationStatus`, or `MessageType`; `null` when absent, `Unknown` when present but not recognized by this version. The `*Raw` property holds the exact text as a nullable `String` |
| Invoice flags | `Int` bitmask taken by `InvoiceBuilder.setFlags` and held by `InvoiceData.flags`; combine `InvoiceFlag` bits from its `bit` property (`ThirdParty`, `Nominal`, `Export`, `Summary`, `SelfBilled`) |
| Unsigned 64-bit counter | `BigInteger`, range 0 through 2^64−1 |
| Binary input / copied output | JVM byte array; copies survive closing their owner |
| Native failure | `FatooraException` subclass with `code`, `kind`, `message`, `detailsJson` |

Close objects returned by methods as well as those you construct. Native errors
are copied to managed data and freed immediately; exceptions need no cleanup.
`Bytes.toByteArray()` copies bytes under the owner lock, so the returned array survives
closing its native owner. Strings reject lone UTF-16 surrogates and embedded NUL.

Calls acquire all participating owners in a fixed lock order. Concurrent calls
on the same object serialize; `close()` waits for active calls and is idempotent.
Later calls throw `IllegalStateException`. Operations on independent owners can
run concurrently. Consumed builders/invoices remain closeable but reject further
use with a native error. Automatic cleanup is nondeterministic; explicitly close owners. Freeing native
memory does not guarantee secret erasure.

Import public classes from `com.fatoora`. 0.7 removed the getter functions,
`String` amount overloads, renamed aliases and `Boolean` coroutine overloads
deprecated in 0.6; see the [0.7 migration guide](../../guides/migrate-0.7/kotlin.md).

## Amounts

Quantities, prices, rates and amounts are `java.math.BigDecimal` in both directions.
The binding sends `toPlainString()` and parses results with `BigDecimal(String)`, so
no digits are lost and exponents never reach the native layer. Returned values are
normalized, so `10.50` reads back as `10.5`; compare with `compareTo`.

=== "Kotlin"

    ```kotlin
    --8<-- "bindings/kotlin/examples/SdkExamples.kt:amounts"
    ```

=== "Java"

    ```java
    --8<-- "bindings/kotlin/examples/JavaExamples.java:amounts"
    ```

## Errors

Native failures throw a subclass of `FatooraException` chosen by error code:

| Code | `ErrorKind` | Exception |
| --- | --- | --- |
| 1 | `InvalidInput` | `InvalidInputException` |
| 2 | `Validation` | `ValidationException` |
| 3 | `Parse` | `ParseException` |
| 4 | `Xml` | `XmlException` |
| 5 | `Crypto` | `CryptoException` |
| 6 | `Io` | `IoException` |
| 7 | `Network` | `NetworkException` |
| 8 | `Unauthorized` | `UnauthorizedException` |
| 9 | `Internal` | `InternalException` |
| 10 | `Api` | `ApiException` |

A code this version does not know throws plain `FatooraException` with a `null`
`kind`. The code, kind, message, and JSON details are managed values; exceptions
need no cleanup. Invalid UTF-16 and embedded NUL are rejected. Use
[structured error details](../errors.md) to distinguish validation, input,
cryptography, and gateway failures.

=== "Kotlin"

    ```kotlin
    try {
        Address.create("XX", "Riyadh", "King Fahd Road", "1234", "12345").use { }
    } catch (error: InvalidInputException) {
        println("Rejected input: ${error.message}")
    } catch (error: FatooraException) {
        println("${error.kind} (${error.code}): ${error.message}")
        println(error.detailsJson)
    }
    ```

=== "Java"

    ```java
    try (Address address = Address.create("XX", "Riyadh", "King Fahd Road", "1234", "12345", null, null, null)) {
        // An unrecognized country code throws before this block.
    } catch (InvalidInputException error) {
        System.err.println("Rejected input: " + error.getMessage());
    } catch (FatooraException error) {
        System.err.println(error.getKind() + " (" + error.getCode() + "): " + error.getMessage());
        System.err.println(error.getDetailsJson());
    }
    ```

## Local validation

| Method | Inputs | Returns |
| --- | --- | --- |
| `Xml.validate(xml)` | XML string | `true` on schema success; throws for malformed XML or schema rejection |
| `Xml.validateZatca(xml, optionsJson)` | XML string, nullable options JSON | Owned `ZatcaValidationReport`, including rejection or incomplete coverage |
| `Xml.hash(xml)` | XML string | Base64 invoice hash string |

Pass `null` for default options, or a JSON object string with
`previous_invoice_hash` and/or `evaluated_at`. The timestamp uses RFC3339 format;
omission uses one native UTC clock snapshot. Unknown option fields are rejected.
Inspect the `isValid` and `isComplete` properties and `stage(i)` on the returned report (Java: `isValid()`, `isComplete()`), or read `toJson()`. Missing predecessor
context makes coverage incomplete. Execution failures throw and may carry a
partial report in their JSON details. See the [validation reference](../invoice-validation.md).

Schemas are bundled; validation needs no external SDK or network. Local
validation does not establish gateway acceptance or certificate issuer trust.

## API requests

Create a client with `ZatcaClient.create(config)`, or with
`ZatcaClient.withTimeout(config, timeout)` to bound each request from connecting
until the response body is read. `timeout` is a `java.time.Duration`, or in
Kotlin also a `kotlin.time.Duration`; it is rounded up to whole milliseconds, and
a request that exceeds it throws `NetworkException`. A zero, negative, or
longer-than-4294967.295-second timeout throws `IllegalArgumentException`.

=== "Kotlin"

    ```kotlin
    import kotlin.time.Duration.Companion.seconds

    val client = ZatcaClient.withTimeout(config, 30.seconds)
    ```

=== "Java"

    ```java
    import java.time.Duration;

    ZatcaClient client = ZatcaClient.withTimeout(config, Duration.ofSeconds(30));
    ```

Native HTTP methods block the calling thread and return objects that must be
closed:

| Method | Inputs | Returns |
| --- | --- | --- |
| `requestComplianceCsid(csr, otp)` | `Csr`, OTP string | `ComplianceCsid` |
| `requestProductionCsid(credentials)` | `ComplianceCsid` | `ProductionCsid` |
| `renewCsid(credentials, csr, otp, acceptLanguage)` | `ProductionCsid`, `Csr`, OTP, nullable language | `ProductionCsid` |
| `checkInvoiceCompliance(invoice, credentials)` | `SignedInvoice`, `ComplianceCsid` | `ValidationResponse` |
| `reportSimplifiedInvoice(invoice, credentials, clearanceMode, acceptLanguage)` | `SignedInvoice`, `ProductionCsid`, `ClearanceMode`, nullable language | `ValidationResponse` |
| `clearStandardInvoice(invoice, credentials, clearanceMode, acceptLanguage)` | `SignedInvoice`, `ProductionCsid`, `ClearanceMode`, nullable language | `ValidationResponse` |

The final `acceptLanguage` argument can be omitted; its default is `null`.

`clearanceMode` controls the request header; use `ClearanceMode.Disabled` for
ordinary reporting and `ClearanceMode.Enabled` for clearance. The coroutine
`*Async` helpers take the same `ClearanceMode`. `acceptLanguage` controls the language header; pass
`null` for no override. It does not describe the response outcome.

A returned response is not necessarily accepted. Check the `outcome` property or
call `ensureAccepted()`. `clearedInvoiceXml` is a nullable `String`; save the
cleared XML when present. See the [API reference](../api-client.md).

=== "Kotlin"

    Coroutine extensions `requestComplianceCsidAsync`, `requestProductionCsidAsync`,
    `renewCsidAsync`, `checkInvoiceComplianceAsync`, `reportSimplifiedInvoiceAsync`,
    and `clearStandardInvoiceAsync` dispatch blocking calls to `Dispatchers.IO`.

    ```kotlin
    // Inside a suspend function, with live client, invoice, and credentials.
    client.reportSimplifiedInvoiceAsync(invoice, credentials, ClearanceMode.Disabled).use { response ->
        response.ensureAccepted()
    }
    ```

    Keep input owners open until completion. Cancellation cannot interrupt an
    in-flight native call. If cancellation discards a completed result, the
    coroutine wrapper closes it. Use a prepared request with a cancellable HTTP
    transport when you need to interrupt network work.

=== "Java"

    Blocking methods can run on your application's executor. Cancelling a task
    does not interrupt an in-flight native HTTP call. Keep its input owners open
    until completion. Use prepared requests with a cancellable HTTP transport
    when you need to interrupt network work.

### Prepared requests

`prepareReportSimplifiedInvoice`, for example, returns an owned `InvoiceRequest`:
read its `url`, `method` and `headersJson` properties and `body()`, send exactly those values,
then call `decode(status, responseBytes)` and `ensureAccepted()` on the returned
response. HTTP status is a checked integer; the response body is a byte array.
Pass non-2xx responses through decoding too, and disable redirects. Close the
prepared request, its body owner, and decoded response after use. Prepared requests own their bytes and decoding context independently
of the original credentials/invoice. Headers contain credentials: do not log them.

## Android

Use the Android artifact alone. It supplies JNA's Android dependency and shrinker
rules and declares the Internet permission. It supports 64-bit ABIs with 16 KiB
ELF alignment. Run network, signing, and validation work off the UI thread.

Android native TLS uses bundled Mozilla/WebPKI roots. Android user-installed CAs
and app network-security configuration do not modify that trust store. Prepared
requests let apps use their own HTTP client and certificate policy.

## License

Original SDK code is `MIT OR Apache-2.0`. Native packages contain ZATCA-derived
rule predicates and messages under their own terms and include
`THIRD_PARTY_NOTICES.md` and `licenses/zatca/`. See the
[third-party notices](https://github.com/mqqz/fatoority/blob/main/THIRD_PARTY_NOTICES.md).
