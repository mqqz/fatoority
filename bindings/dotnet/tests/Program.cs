using System;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Xml.Linq;
using System.Threading;
using System.Threading.Tasks;
using Fatoora;

static void Check(bool value, string message)
{
    if (!value) throw new Exception(message);
}
static void Throws<T>(Action action) where T : Exception
{
    try { action(); } catch (T) { return; }
    throw new Exception($"Expected {typeof(T).Name}");
}
static void NativeError(Action action, int code, string? message = null)
{
    try { action(); }
    catch (BindingErrorException error)
    {
        using var inner = error.Inner;
        Check(inner.Code() == code, $"Error code: {inner.Code()} != {code}");
        Check(error.Message == inner.Message(), "Exception lost native message");
        if (message != null) Check(error.Message.Contains(message), "Unexpected error message");
        using var details = JsonDocument.Parse(inner.DetailsJson());
        return;
    }
    throw new Exception("Expected native error");
}
static byte[] Copy(Bytes bytes)
{
    using (bytes)
    using (var span = bytes.AsSlice()) return span.Clone();
}
var root = args[0];
using var absent = Address.New("SA", "الرياض", "Street", "1234", "12345", null, null, null);
Check(absent.City() == "الرياض", "UTF-8 round trip");
Check(absent.AdditionalStreet() is null, "Null optional string");
using var present = Address.New("SA", "Riyadh", "Street", "1234", "12345", "", null, "حي العليا");
using var empty = present.AdditionalStreet();
using var district = present.District();
Check(empty?.Value() == "" && district?.Value() == "حي العليا", "Optional string round trip");
foreach (var invalid in new[] { "\uD800", "\uDC00" })
{
    Throws<EncoderFallbackException>(() => Address.New("SA", invalid, "s", "1234", "12345", null, null, null));
    Throws<EncoderFallbackException>(() => Address.New("SA", "c", "s", "1234", "12345", invalid, null, null));
}
NativeError(() => Address.New("XX", "a", "b", "1", "1", null, null, null), 1, "Invalid country code");
NativeError(() => Address.New("SA", "bad\0city", "s", "1234", "12345", null, null, null), 1, "NUL");
absent.Dispose(); absent.Dispose();
Throws<ObjectDisposedException>(() => absent.City());

using var key = SigningKey.Generate();
var pem = key.ToPem();
using var restored = SigningKey.FromPem(pem);
Check(restored.ToPem() == pem, "PEM round trip");
var der = Copy(key.ToDer());
using var restoredDer = SigningKey.FromDer(der);
Check(restoredDer.ToPem() == pem, "DER round trip");
using var bytes = key.ToDer();
using var borrowed = bytes.AsSlice();
borrowed.WithSpan(s => {
    bytes.Dispose();
    Check(s.SequenceEqual(der), "Active read survives owner disposal");
    Throws<InvalidOperationException>(() => borrowed.Dispose());
});
Throws<InvalidOperationException>(() => borrowed.Clone());
borrowed.Dispose();
Throws<ObjectDisposedException>(() => borrowed.Clone());
key.Dispose();
Throws<ObjectDisposedException>(() => key.ToPem());
NativeError(() => SigningKey.FromDer(new byte[] { 1, 2 }), 1);

using var builder = InvoiceBuilder.New(0, 0, null, null, null, null);
NativeError(() => builder.Build(), 2);
NativeError(() => builder.SetId("again"), 1, "consumed");

var corpus = Path.Combine(root, "fatoora-core/tests/fixtures/sdk-parity");
using var signer = Signer.FromDer(File.ReadAllBytes(Path.Combine(corpus, "credentials/certificate.der")), File.ReadAllBytes(Path.Combine(corpus, "credentials/private-key.der")));
Check(Copy(signer.CertificateDer()).SequenceEqual(File.ReadAllBytes(Path.Combine(corpus, "credentials/certificate.der"))), "Certificate fixture");
using var config = Config.New(0);
using var client = ZatcaClient.Create(config);
using var credentials = CsidProduction.Create(0, null, "token", "secret");
foreach (var kind in new[] { "simplified", "standard" })
{
    var fixture = Path.Combine(corpus, $"cases/{kind}-invoice");
    var xml = File.ReadAllText(Path.Combine(fixture, "input.xml"));
    using var expected = JsonDocument.Parse(File.ReadAllText(Path.Combine(fixture, "expected.json")));
    Check(Xml.Hash(xml) == expected.RootElement.GetProperty("hash").GetString(), "Frozen SDK hash parity");
    using var imported = SignedInvoice.FromXml(File.ReadAllText(Path.Combine(fixture, "sdk-signed.xml")));
    Check(imported.HashBase64() == Xml.Hash(imported.Xml()), "Imported signed XML hash parity");
    using var invoice = FinalizedInvoice.FromXml(xml);
    using var signed = signer.Sign(invoice);
    NativeError(() => invoice.Xml(), 1, "consumed");
    using var data = signed.Data();
    using var totals = signed.Totals();
    var inputTotal = XDocument.Parse(xml).Descendants().Single(e => e.Name.LocalName == "LegalMonetaryTotal").Elements().Single(e => e.Name.LocalName == "PayableAmount").Value;
    Check(decimal.Parse(totals.PayableAmount(), System.Globalization.CultureInfo.InvariantCulture) == decimal.Parse(inputTotal, System.Globalization.CultureInfo.InvariantCulture), "Invoice totals match XML");
    using var request = kind == "simplified"
        ? client.PrepareReportSimplifiedInvoice(signed, credentials, false, null)
        : client.PrepareClearStandardInvoice(signed, credentials, true, "ar");
    Check(request.Method() == "POST", "Prepared method");
    using var headers = JsonDocument.Parse(request.HeadersJson());
    using var body = JsonDocument.Parse(Copy(request.Body()));
    Check(body.RootElement.GetProperty("invoiceHash").GetString() == signed.HashBase64(), "Prepared hash");
    Check(Encoding.UTF8.GetString(Convert.FromBase64String(body.RootElement.GetProperty("invoice").GetString()!)) == signed.Xml(), "Prepared invoice");
    var signedXml = signed.IntoXml();
    NativeError(() => signed.Xml(), 1, "consumed");
    signed.Dispose();
    Check(data.Id().Length != 0 && totals.PayableAmount().Length != 0, "Owned snapshots survive source");
    var responseBody = kind == "simplified"
        ? "{\"validationResults\":{\"status\":\"PASS\",\"warningMessages\":[{\"code\":\"WARN\"}]},\"reportingStatus\":\"REPORTED\"}"
        : JsonSerializer.Serialize(new { validationResults = new { status = "PASS", warningMessages = new[] { new { code = "WARN" } } }, clearanceStatus = "CLEARED", clearedInvoice = Convert.ToBase64String(Encoding.UTF8.GetBytes(signedXml)) });
    using var response = request.Decode(200, Encoding.UTF8.GetBytes(responseBody));
    Check(response.HttpStatus() == 200 && response.Outcome() == InvoiceOutcome.Accepted, "Response outcome/status");
    response.EnsureAccepted();
    using var results = response.ValidationResults();
    response.Dispose();
    using var warning = results.WarningMessage(0);
    results.Dispose();
    using var code = warning.Code();
    Check(code?.Value() == "WARN", "Response snapshots survive disposal");
    NativeError(() => request.Decode(401, Encoding.UTF8.GetBytes("{}")), 8);
    NativeError(() => request.Decode(200, Encoding.UTF8.GetBytes("invalid json")), 3);
}
using var compliance = CsidCompliance.Create(0, "request-id", "token", "secret");
using var productionRequest = client.PreparePostCcsidForPcsid(compliance);
compliance.Dispose(); client.Dispose();
Check(productionRequest.Method() == "POST", "Request owns dependencies");
using var production = productionRequest.Decode(200, Encoding.UTF8.GetBytes("{\"requestID\":123,\"binarySecurityToken\":\"new-token\",\"secret\":\"new-secret\"}"));
Check(production.Secret() == "new-secret", "Credential decode");
Console.WriteLine("C# contracts passed: strings, errors, disposal, crypto, signing, SDK hash parity, requests, responses");
