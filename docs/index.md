---
title: Create Saudi E-Invoices - ZATCA SDK and CLI
description: Create Saudi e-invoices from application data. Calculate VAT and totals, generate UBL XML, validate and sign locally, and submit through ZATCA APIs.
---

# Create Saudi e-invoices in your application

fatoority is an open-source SDK and CLI for Saudi e-invoicing. Build standard and
simplified invoices from application data, including credit notes, debit notes,
and prepayment invoices. Calculate totals and VAT, generate UBL XML, then sign
and validate invoices locally before submitting them through the ZATCA APIs.

Use the Rust library, add it to your application in Python, JavaScript/TypeScript,
C#, Kotlin/Java (including Android), or C/C++, or run the command-line tool from your terminal.

[Run the sandbox walkthrough](guides/sandbox-walkthrough.md){ .md-button .md-button--primary }
[Choose your language](#choose-your-language){ .md-button }
[View on GitHub](https://github.com/mqqz/fatoority)
{ .home-actions }

fatoority is an independent project and is not affiliated with ZATCA.

Start with one [complete example in Python, C#, or TypeScript](guides/sandbox-walkthrough.md)
covering creation through reporting, with a diagram of the ZATCA journey.

## What you can build

| Task | How fatoority helps |
| --- | --- |
| [Create invoices](guides/invoice-creation.md) | Build invoice data with calculated totals and UBL XML output. Choose standard or simplified tax invoices, credit/debit notes, or prepayments. |
| [Handle pricing and VAT](guides/invoice-recipes.md) | Apply line or document discounts and charges, use price-base quantities, and group VAT by category and rate with exact decimal arithmetic. |
| [Include business references](reference/invoice-model.md) | Carry purchase orders, contracts, delivery dates, payment instructions/accounts, and party/item identifiers into XML. |
| [Read existing XML](reference/xml.md) | Inspect supported fields and totals. Signed import checks cryptographic integrity and QR consistency while retaining the original XML. |
| [Validate locally](guides/validation.md) | Check UBL schemas and the pinned SDK's 257 business-rule assertions. Inspect rule findings, warnings, and incomplete stages before submission. |
| [Sign invoices](guides/invoice-signing.md) | Sign with a matching certificate and private key. Compute invoice hashes and prepare API payloads. |
| [Generate QR payloads](guides/qr.md) | Generate base64 invoice QR data or read the payload embedded in signed XML. |
| [Onboard and submit](guides/api.md) | Generate a [CSR](guides/csr.md), obtain or renew credentials, run compliance checks, and integrate reporting or clearance. |

Local invoice processing needs no official SDK installation or network access.
The Rust core has no Java dependency.
The JavaScript package includes WebAssembly for Node.js and browsers; Kotlin/Java
also has an Android artifact. Use prepared API requests when your application
needs its own HTTP transport.

Local validation helps catch problems before submission. It does not establish
ZATCA gateway acceptance or certificate issuer trust. See the
[validation guide](guides/validation.md) for coverage and how to interpret results.

## Try it: validate an invoice

Start with a local XML schema check. You do not need an account, certificate, or
API credentials for this example.

### 1. Install the CLI

With [Rust and Cargo](https://www.rust-lang.org/tools/install) installed, run:

```sh
cargo install fatoora-cli
```

You can also download a binary for your platform from
[GitHub Releases](https://github.com/mqqz/fatoority/releases). Extract the archive
and put the `fatoora` executable on your `PATH`.

### 2. Download the sample invoice

Save [sample-invoice.xml](assets/examples/sample-invoice.xml) in your working
folder. It contains demonstration data for this schema check.

### 3. Run the check

From the folder containing the file:

```sh
fatoora validate --invoice sample-invoice.xml
```

Expected output:

```text
OK
```

`OK` means the XML passes the bundled UBL schema check. To run local ZATCA rules
and inspect individual findings, follow the [invoice validation guide](guides/validation.md).
For more terminal commands, see the [CLI guide](guides/cli.md).

## Choose your language

Install the package for your application, then follow its examples. The project
and most packages are named `fatoority`; existing API names such as Python's
`fatoora` import and Rust's `fatoora_core` remain unchanged.

=== "Rust"
    ```sh
    cargo add fatoora-core
    ```

    Import `fatoora_core` in your code.
    [Create your first invoice](guides/invoice-creation.md) or browse the
    [Rust API reference](https://docs.rs/fatoora-core/latest/fatoora_core/).

=== "Python"
    ```sh
    pip install fatoority
    ```

    Import `fatoora` in your code.
    [Follow the Python setup and examples](reference/bindings/python.md).

=== "JavaScript"
    ```sh
    npm install fatoority
    ```

    Use the WebAssembly package in Node.js or a browser.
    [Follow the JavaScript/TypeScript setup and examples](reference/bindings/javascript.md),
    including browser asset setup and API transport requirements.

=== "C#"
    ```sh
    dotnet add package Fatoority
    ```

    Use the `Fatoora` namespace with .NET 8 or later.
    [Follow the C# setup and examples](reference/bindings/csharp.md), including
    supported native platforms.

=== "Kotlin / Java / Android"
    Install from Maven Central with Gradle or Maven.

    **Gradle** (`build.gradle.kts`):

    ```kotlin
    repositories { mavenCentral() }
    dependencies {
        implementation("com.fatoority:fatoority:0.5.0")
    }
    ```

    **Maven** (add inside `<dependencies>` in `pom.xml`):

    ```xml
    <dependency>
      <groupId>com.fatoority</groupId>
      <artifactId>fatoority</artifactId>
      <version>0.5.0</version>
    </dependency>
    ```

    Import `com.fatoora` in your code.
    [Follow the Kotlin/Java examples](reference/bindings/kotlin.md) for
    supported platforms and Android setup. For Android, use
    `com.fatoority:fatoority-android:0.5.0` instead of the desktop artifact.

=== "C / C++"
    Download the `fatoora-ffi-*` archive with the native library and C/C++ headers from
    [GitHub Releases](https://github.com/mqqz/fatoority/releases).

    [Follow the C/C++ setup and examples](reference/bindings/c.md) for compilation
    and linking instructions.

## Continue your integration

If you are adding invoicing to an application, start with
[creating an invoice](guides/invoice-creation.md). The
[business-case recipes](guides/invoice-recipes.md) cover pricing and document types.
Then [sign](guides/invoice-signing.md) and [validate](guides/validation.md) the XML
before sending it.

For device onboarding, [generate a CSR](guides/csr.md), then use the
[ZATCA API client](guides/api.md) to request credentials and submit invoices.
The guides explain the inputs needed at each step.

## About the project

The core is written in Rust and shared by the CLI and language bindings.
The project code is available under the
[MIT or Apache-2.0 license](https://github.com/mqqz/fatoority#license).
Third-party materials retain their upstream terms; see the
[third-party notices](https://github.com/mqqz/fatoority/blob/main/THIRD_PARTY_NOTICES.md).

See the [SDK compatibility notes](development/sdk-parity.md) for the scope of
comparison with the official SDK, and [benchmarks](benchmarks.md) for measured
performance and methodology.

Need help with an integration? [Open an issue](https://github.com/mqqz/fatoority/issues).
To help improve the library or these docs, read the [contributing guide](contributing.md).
