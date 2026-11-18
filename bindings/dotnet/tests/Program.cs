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
    catch (FatooraException error)
    {
        Check(error.Code == code, $"Error code: {error.Code} != {code}");
        Check(error.Kind == (ErrorKind)code, "Error kind");
        Check(error.GetType() == FatooraException.FromCode(code, "").GetType(), $"Exception type: {error.GetType().Name}");
        Check(error.GetType().Name == $"{error.Kind}Exception", $"Exception type: {error.GetType().Name}");
        if (message != null) Check(error.Message.Contains(message), "Unexpected error message");
        using var details = JsonDocument.Parse(error.DetailsJson);
        Check(details.RootElement.ValueKind == JsonValueKind.Object, "Error details object");
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
using var absent = Address.Create("SA", "الرياض", "Street", "1234", "12345", null, null, null);
Check(absent.City == "الرياض", "UTF-8 round trip");
Check(absent.AdditionalStreet is null, "Null optional string");
using var present = Address.Create("SA", "Riyadh", "Street", "1234", "12345", "", null, "حي العليا");
Check(present.AdditionalStreet == "" && present.District == "حي العليا", "Optional string round trip");
Check(present.Province is null, "Null optional string return");
using (var withProvince = present.WithProvince("منطقة الرياض"))
    Check(withProvince.Province == "منطقة الرياض", "Optional string return");
foreach (var invalid in new[] { "\uD800", "\uDC00" })
{
    Throws<EncoderFallbackException>(() => Address.Create("SA", invalid, "s", "1234", "12345", null, null, null));
    Throws<EncoderFallbackException>(() => Address.Create("SA", "c", "s", "1234", "12345", invalid, null, null));
}
NativeError(() => Address.Create("XX", "a", "b", "1", "1", null, null, null), 1, "Invalid country code");
NativeError(() => Address.Create("SA", "bad\0city", "s", "1234", "12345", null, null, null), 1, "NUL");
foreach (var kind in Enum.GetValues<ErrorKind>())
{
    var mapped = FatooraException.FromCode((int)kind, "m");
    Check(mapped.GetType().Name == $"{kind}Exception" && mapped.Kind == kind && mapped.Code == (int)kind, $"Exception for {kind}");
}
var unknown = FatooraException.FromCode(99, "future", "{\"type\":\"future\"}");
Check(unknown.GetType() == typeof(FatooraException) && unknown.Kind == null && unknown.DetailsJson == "{\"type\":\"future\"}", "Unknown error code");
absent.Dispose(); absent.Dispose();
Throws<ObjectDisposedException>(() => _ = absent.City);

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
Check(borrowed.Clone().SequenceEqual(der), "Copy survives owner disposal");
borrowed.Dispose();
Throws<ObjectDisposedException>(() => borrowed.Clone());
key.Dispose();
Throws<ObjectDisposedException>(() => key.ToPem());
NativeError(() => SigningKey.FromDer(new byte[] { 1, 2 }), 1);
NativeError(() => SigningKey.FromPem("invalid key"), 1); // Matches the errors example.

// Decimals cross the boundary exactly and independent of the current culture.
var culture = System.Globalization.CultureInfo.CurrentCulture;
System.Globalization.CultureInfo.CurrentCulture = new System.Globalization.CultureInfo("de-DE");
using (var line = InvoiceLineItem.Create("item", 1.50m, "PCE", 0.0000000000000000000000000001m, 15m, VatCategory.Standard))
{
    Check(line.Quantity == 1.5m && line.UnitPrice == 0.0000000000000000000000000001m && line.VatRate == 15m, "Decimal round trip");
    Check(line.ItemGrossPrice is null && line.VatCategory == VatCategory.Standard, "Optional decimal and enum properties");
}
using (var percent = Adjustment.PercentOf(-1234567890.123456789m, 12.5m))
    Check(percent.BaseAmount == -1234567890.123456789m && percent.Percentage == 12.5m, "Optional decimal round trip");
using (var rebate = Adjustment.PercentOf(250m, 10m))
using (var supplied = rebate.SuppliedAmount(25m))
    Check(rebate.Percentage == 10m && supplied.Value == 25m, "Supplied adjustment amount");
System.Globalization.CultureInfo.CurrentCulture = culture;

using var builder = InvoiceBuilder.Create(InvoiceTypeKind.Tax, InvoiceSubType.Standard, null, null, null, null);
NativeError(() => builder.Build(), 2);
NativeError(() => builder.SetId("again"), 1, "consumed");

var corpus = Path.Combine(root, "fatoora-core/tests/fixtures/sdk-parity");
using var signer = InvoiceSigner.FromDer(File.ReadAllBytes(Path.Combine(corpus, "credentials/certificate.der")), File.ReadAllBytes(Path.Combine(corpus, "credentials/private-key.der")));
Check(Copy(signer.CertificateDer()).SequenceEqual(File.ReadAllBytes(Path.Combine(corpus, "credentials/certificate.der"))), "Certificate fixture");
using var config = Config.Create(ZatcaEnvironment.NonProduction);
Check(config.Env == ZatcaEnvironment.NonProduction, "Environment round trip");
using var client = ZatcaClient.Create(config);
ZatcaClient.WithTimeout(config, TimeSpan.FromSeconds(30)).Dispose();
ZatcaClient.WithTimeout(config, TimeSpan.FromTicks(1)).Dispose(); // Rounds up to 1 ms.
ZatcaClient.WithTimeout(config, TimeSpan.FromMilliseconds(uint.MaxValue)).Dispose();
foreach (var bad in new[] { TimeSpan.Zero, TimeSpan.FromTicks(-1), Timeout.InfiniteTimeSpan,
                            TimeSpan.FromMilliseconds(uint.MaxValue) + TimeSpan.FromTicks(1), TimeSpan.MaxValue })
{
    Throws<ArgumentOutOfRangeException>(() => ZatcaClient.WithTimeout(config, bad));
}
Throws<ArgumentNullException>(() => ZatcaClient.WithTimeout(null!, TimeSpan.FromSeconds(1)));
Check(typeof(ZatcaClient).GetMethods().Where(m => m.Name == "WithTimeout")
        .Select(m => m.GetParameters()[1].ParameterType).SequenceEqual(new[] { typeof(TimeSpan) }),
      "TimeSpan is the only public timeout overload");
using var credentials = ProductionCsid.Create(ZatcaEnvironment.NonProduction, null, "token", "secret");
foreach (var kind in new[] { "simplified", "standard" })
{
    var fixture = Path.Combine(corpus, $"cases/{kind}-invoice");
    var xml = File.ReadAllText(Path.Combine(fixture, "input.xml"));
    using var expected = JsonDocument.Parse(File.ReadAllText(Path.Combine(fixture, "expected.json")));
    Check(Xml.Hash(xml) == expected.RootElement.GetProperty("hash").GetString(), "Frozen SDK hash parity");
    using var imported = SignedInvoice.FromXml(File.ReadAllText(Path.Combine(fixture, "sdk-signed.xml")));
    Check(imported.Hash == Xml.Hash(imported.Xml), "Imported signed XML hash parity");
    using var invoice = FinalizedInvoice.FromXml(xml);
    using var signed = signer.Sign(invoice);
    NativeError(() => _ = invoice.Xml, 1, "consumed");
    using var data = signed.Data();
    using var totals = signed.Totals();
    var inputTotal = XDocument.Parse(xml).Descendants().Single(e => e.Name.LocalName == "LegalMonetaryTotal").Elements().Single(e => e.Name.LocalName == "PayableAmount").Value;
    Check(totals.PayableAmount == decimal.Parse(inputTotal, System.Globalization.CultureInfo.InvariantCulture), "Invoice totals match XML");
    using var request = kind == "simplified"
        ? client.PrepareReportSimplifiedInvoice(signed, credentials, ClearanceMode.Disabled, null)
        : client.PrepareClearStandardInvoice(signed, credentials, ClearanceMode.Enabled, "ar");
    Check(request.Method == "POST", "Prepared method");
    using var headers = JsonDocument.Parse(request.HeadersJson);
    using var body = JsonDocument.Parse(Copy(request.Body()));
    Check(body.RootElement.GetProperty("invoiceHash").GetString() == signed.Hash, "Prepared hash");
    Check(Encoding.UTF8.GetString(Convert.FromBase64String(body.RootElement.GetProperty("invoice").GetString()!)) == signed.Xml, "Prepared invoice");
    var signedXml = signed.IntoXml();
    NativeError(() => _ = signed.Xml, 1, "consumed");
    signed.Dispose();
    Check(data.Id.Length != 0 && totals.PayableAmount > 0, "Owned snapshots survive source");
    var responseBody = kind == "simplified"
        ? "{\"validationResults\":{\"status\":\"PASS\",\"warningMessages\":[{\"code\":\"WARN\"}]},\"reportingStatus\":\"REPORTED\"}"
        : JsonSerializer.Serialize(new { validationResults = new { status = "PASS", warningMessages = new[] { new { code = "WARN" } } }, clearanceStatus = "CLEARED", clearedInvoice = Convert.ToBase64String(Encoding.UTF8.GetBytes(signedXml)) });
    using var response = request.Decode(200, Encoding.UTF8.GetBytes(responseBody));
    Check(response.HttpStatus == 200 && response.Outcome == InvoiceOutcome.Accepted, "Response outcome/status");
    response.EnsureAccepted();
    if (kind == "simplified")
    {
        Check(response.ReportingStatus == ReportingStatus.Reported && response.ClearanceStatus == null, "Reporting status enum");
        Check(response.ReportingStatusRaw == "REPORTED", "Reporting status wire value");
    }
    else
    {
        Check(response.ClearanceStatus == ClearanceStatus.Cleared, "Clearance status enum");
    }
    using var results = response.ValidationResults();
    Check(results.Status == ValidationStatus.Pass, "Validation status enum");
    response.Dispose();
    using var warning = results.WarningMessage(0);
    results.Dispose();
    Check(warning.Code == "WARN", "Response snapshots survive disposal");
    NativeError(() => request.Decode(401, Encoding.UTF8.GetBytes("{}")), 8);
    NativeError(() => request.Decode(200, Encoding.UTF8.GetBytes("invalid json")), 3);
}
using var compliance = ComplianceCsid.Create(ZatcaEnvironment.NonProduction, "request-id", "token", "secret");
using var productionRequest = client.PrepareRequestProductionCsid(compliance);
compliance.Dispose(); client.Dispose();
Check(productionRequest.Method == "POST", "Request owns dependencies");
using var production = productionRequest.Decode(200, Encoding.UTF8.GetBytes("{\"requestID\":123,\"binarySecurityToken\":\"new-token\",\"secret\":\"new-secret\"}"));
Check(production.Secret == "new-secret", "Credential decode");
Check(production.RequestId == "123", "Decoded request id");
using (var anonymous = ComplianceCsid.Create(ZatcaEnvironment.Simulation, null, "token", "secret"))
    Check(anonymous.RequestId == null, "Absent request id is null");
using (var report = Xml.ValidateZatca("<wrong/>", "{\"evaluated_at\":\"2026-09-23T12:00:00+03:00\"}"))
{
    Check(!report.IsValid && report.HasErrors, "Typed ZATCA report outcome");
    Check(report.EvaluatedAt == "2026-09-23T12:00:00+03:00", "Typed ZATCA report clock");
    Check(report.StagesLen == 6, "Typed ZATCA report stages");
    using var stage = report.Stage(0);
    Check(stage.Stage == ZatcaStage.Xsd && stage.Status == ZatcaStageStatus.Completed, "Typed ZATCA stage");
    using var finding = stage.Finding(0);
    Check(finding.Code == "XSD_INVALID" && finding.Severity == Severity.Error, "Typed ZATCA finding");
    using var json = JsonDocument.Parse(report.ToJson());
    Check(!json.RootElement.GetProperty("is_valid").GetBoolean(), "ZATCA report JSON");
}
Console.WriteLine("C# contracts passed: strings, decimals, properties, errors, disposal, crypto, signing, SDK hash parity, requests, responses, validation reports");
