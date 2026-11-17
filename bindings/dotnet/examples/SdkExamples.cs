using System;
using System.Text.Json;
using Fatoora;

public static class SdkExamples
{
    // --8<-- [start:csr]
    public static (string CsrPem, string KeyPem) GenerateCsr(string propertiesText)
    {
        using var properties = CsrProperties.FromStr(propertiesText);
        using var key = SigningKey.Generate();
        using var csr = properties.Build(key, ZatcaEnvironment.NonProduction);
        return (csr.ToPem(), key.ToPem());
    }
    // --8<-- [end:csr]

    // --8<-- [start:signing]
    public static string SignInvoice(string xml, string certificatePem, string keyPem)
    {
        using var signer = InvoiceSigner.FromPem(certificatePem, keyPem);
        using var invoice = FinalizedInvoice.FromXml(xml);
        using var signed = signer.Sign(invoice); // Consumes invoice's inner state.
        return signed.IntoXml();
    }
    // --8<-- [end:signing]

    // --8<-- [start:validation]
    public static ZatcaValidationReport ValidateInvoice(
        string xml, string previousHash, string evaluatedAt)
    {
        var options = JsonSerializer.Serialize(new
        {
            previous_invoice_hash = previousHash,
            evaluated_at = evaluatedAt
        });
        return Xml.ValidateZatca(xml, options);
    }
    // The caller disposes the report and inspects IsValid(), IsComplete() and
    // Stage(i); ToJson() returns the same report as JSON.
    // --8<-- [end:validation]

    // --8<-- [start:qr]
    public static string ReadQr(string signedXml)
    {
        using var invoice = SignedInvoice.FromXml(signedXml);
        return invoice.QrCode(); // Base64-encoded TLV payload.
    }
    // --8<-- [end:qr]

    // --8<-- [start:api]
    public static void ReportInvoice(string signedXml, string token, string secret)
    {
        using var config = Config.New(ZatcaEnvironment.Production);
        using var client = ZatcaClient.Create(config);
        using var credentials = ProductionCsid.Create(ZatcaEnvironment.Production, null, token, secret);
        using var invoice = SignedInvoice.FromXml(signedXml);
        using var response = client.ReportSimplifiedInvoice(
            invoice, credentials, ClearanceMode.Disabled, "en");
        response.EnsureAccepted(); // HTTP success alone does not mean acceptance.
        Console.WriteLine(response.Outcome());
    }
    // This method performs a blocking network request.
    // --8<-- [end:api]

    // --8<-- [start:errors]
    public static void ShowError()
    {
        try
        {
            using var key = SigningKey.FromPem("invalid key");
        }
        catch (BindingErrorException error)
        {
            using var details = error.Inner;
            Console.WriteLine($"{details.Code()}: {error.Message}");
            Console.WriteLine(details.DetailsJson());
        }
    }
    // --8<-- [end:errors]
}
