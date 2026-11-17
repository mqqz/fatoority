package examples

import com.fatoora.*

object SdkExamples {
    // --8<-- [start:csr]
    fun generateCsr(propertiesText: String): Pair<String, String> =
        CsrProperties.fromPropertiesStr(propertiesText).use { properties ->
            SigningKey.generate().use { key ->
                properties.build(key, Environment.NonProduction).use { csr -> csr.toPem() to key.toPem() }
            }
        }
    // --8<-- [end:csr]

    // --8<-- [start:signing]
    fun signInvoice(xml: String, certificatePem: String, keyPem: String): String =
        Signer.fromPem(certificatePem, keyPem).use { signer ->
            FinalizedInvoice.fromXml(xml).use { invoice ->
                signer.sign(invoice).use { signed -> signed.intoXml() }
            }
        }
    // --8<-- [end:signing]

    // --8<-- [start:validation]
    fun validateInvoice(xml: String, optionsJson: String? = null): String =
        Config.create(Environment.NonProduction).use { config -> Xml.validateZatca(config, xml, optionsJson) }
    // Inspect the returned JSON's is_valid and stages fields.
    // --8<-- [end:validation]

    // --8<-- [start:qr]
    fun readQr(signedXml: String): String =
        SignedInvoice.fromXml(signedXml).use { invoice -> invoice.qrCode() }
    // --8<-- [end:qr]

    // --8<-- [start:api]
    suspend fun reportInvoice(signedXml: String, token: String, secret: String) {
        Config.create(Environment.Production).use { config ->
            ZatcaClient.create(config).use { client ->
                CsidProduction.create(Environment.Production, null, token, secret).use { credentials ->
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
