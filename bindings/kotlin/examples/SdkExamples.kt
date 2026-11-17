package examples

import com.fatoora.*

object SdkExamples {
    // --8<-- [start:csr]
    fun generateCsr(propertiesText: String): Pair<String, String> =
        CsrProperties.fromStr(propertiesText).use { properties ->
            SigningKey.generate().use { key ->
                properties.build(key, Environment.NonProduction).use { csr -> csr.toPem() to key.toPem() }
            }
        }
    // --8<-- [end:csr]

    // --8<-- [start:signing]
    fun signInvoice(xml: String, certificatePem: String, keyPem: String): String =
        InvoiceSigner.fromPem(certificatePem, keyPem).use { signer ->
            FinalizedInvoice.fromXml(xml).use { invoice ->
                signer.sign(invoice).use { signed -> signed.intoXml() }
            }
        }
    // --8<-- [end:signing]

    // --8<-- [start:validation]
    fun validateInvoice(xml: String, optionsJson: String? = null): Boolean =
        Xml.validateZatca(xml, optionsJson).use { report -> report.isValid() }
    // The report also exposes isComplete(), stage(i) findings and toJson().
    // --8<-- [end:validation]

    // --8<-- [start:qr]
    fun readQr(signedXml: String): String =
        SignedInvoice.fromXml(signedXml).use { invoice -> invoice.qrCode() }
    // --8<-- [end:qr]

    // --8<-- [start:api]
    suspend fun reportInvoice(signedXml: String, token: String, secret: String) {
        Config.create(Environment.Production).use { config ->
            ZatcaClient.create(config).use { client ->
                ProductionCsid.create(Environment.Production, null, token, secret).use { credentials ->
                    SignedInvoice.fromXml(signedXml).use { invoice ->
                        client.reportSimplifiedInvoiceAsync(invoice, credentials, false, "en").use { response ->
                            response.ensureAccepted()
                        }
                    }
                }
            }
        }
    }
    // --8<-- [end:api]
}
