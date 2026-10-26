---
title: Create a Saudi E-Invoice from Application Data
description: Build an invoice with fatoority, calculate VAT and payable totals, and write UBL XML using Rust or Python.
---

# Create an invoice

Build a simplified invoice with two notebooks priced at SAR 50 each, excluding
VAT. fatoority calculates SAR 15 VAT and a SAR 115 payable total, then writes
`invoice.xml`. No certificate or API credentials are needed.

This example uses demonstration seller details, a fixed timestamp, and a sample
previous-invoice hash. Replace them with your own data for an integration. Your
application owns invoice numbering, UUIDs, counters, and predecessor history.
Running the example replaces `invoice.xml` in the working folder.

For one runnable program covering creation through reporting, follow the
[sandbox walkthrough](sandbox-walkthrough.md).

## Run the example

=== "Rust"

    Create a project and add the library:

    ```sh
    cargo new fatoority-create
    cd fatoority-create
    cargo add fatoora-core
    ```

    Save this code as `src/main.rs` and run `cargo run`.
    [Download creation.rs](../assets/examples/rust/creation.rs).

    ```rust
    --8<-- "docs/assets/examples/rust/creation.rs"
    ```

=== "Python"

    Install the package:

    ```sh
    pip install fatoority
    ```

    Save this code as `creation.py` and run `python creation.py`.
    [Download creation.py](../assets/examples/python/creation.py).

    ```python
    --8<-- "docs/assets/examples/python/creation.py"
    ```

Expected output:

```text
VAT: 15 SAR
Payable: 115 SAR
Saved invoice.xml
```

Open the XML to inspect the seller and line item, followed by the VAT breakdown
and monetary totals.
Decimal inputs keep their precision through calculation; XML monetary fields
are written with two decimal places. See [rounding](../reference/numbers.md)
for calculated and supplied amounts.

## Adapt it to your application

Use the [pricing and invoice recipes](invoice-recipes.md) for discounts,
charges, mixed VAT, credit/debit notes, and prepayment adjustments. The
[invoice model reference](../reference/invoice-model.md) covers additional
fields and the language bindings. JavaScript/TypeScript, C#, Kotlin/Java, and
C/C++ expose builders through the same core.

If your application already has XML, use [XML import](../reference/xml.md)
to inspect the supported fields or pass the document to the signing guide.
Unsigned model serialization may normalize formatting and omit unsupported
extensions. Signed import retains the original XML and checks its local integrity.

## Sign and validate

Building checks the model's fields and arithmetic. Run the
[validation guide](validation.md) for schema and local ZATCA rules.
An unsigned document has no signature to verify; after
[signing with your certificate and matching key](invoice-signing.md), validate
that signed document before [submission](api.md). The validation guide uses an
unsigned fixture named `invoice.xml`; to check the document you just signed,
point the CLI at `signed.xml`:

```sh
fatoora validate --invoice signed.xml --profile zatca --format json \
  --previous-invoice-hash "$PREVIOUS_INVOICE_HASH"
```

Set `PREVIOUS_INVOICE_HASH` to the expected predecessor from your invoice history.
For this demonstration, use the sample predecessor supplied to the builder.
Keep `signed.xml` intact when validating or submitting it.

The example creates UBL XML. PDF rendering is on the project roadmap.
