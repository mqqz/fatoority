using System.Text.Json;
using System.Globalization;
using Fatoora;

public static class Program
{
    private const byte Sandbox = 0; // Integration Sandbox only.
    private static readonly JsonSerializerOptions JsonOptions = new() { WriteIndented = true };
    private static string Required(string name) =>
        !string.IsNullOrWhiteSpace(Environment.GetEnvironmentVariable(name))
            ? Environment.GetEnvironmentVariable(name)!.Trim()
            : throw new InvalidOperationException($"Set {name} first");
    private static void Save(string path, object value) =>
        File.WriteAllText(path, JsonSerializer.Serialize(value, JsonOptions) + "\n");
    private static string? ReadText(Text? value)
    {
        using (value) return value?.Value();
    }
    private static object[] Messages(nuint count, Func<nuint, ValidationMessage> get)
    {
        var result = new List<object>();
        for (nuint i = 0; i < count; i++)
        {
            using var message = get(i);
            result.Add(new { code = ReadText(message.Code()), message = ReadText(message.Message()) });
        }
        return result.ToArray();
    }
    public static void HandleResponse(ValidationResponse response, string output)
    {
        using var results = response.ValidationResults();
        var summary = new {
            http_status = response.HttpStatus(), outcome = response.Outcome().ToString(),
            reporting_status = ReadText(response.ReportingStatus()),
            validation_status = ReadText(results.Status()),
            info = Messages(results.InfoLen(), results.InfoMessage),
            warnings = Messages(results.WarningLen(), results.WarningMessage),
            errors = Messages(results.ErrorLen(), results.ErrorMessage),
        };
        Save(Path.Combine(output, "response.json"), summary);
        Console.WriteLine(JsonSerializer.Serialize(summary, JsonOptions));
        response.EnsureAccepted(); // A successful HTTP status alone is insufficient.
        Console.WriteLine("Sandbox reporting accepted. Keep any warnings in response.json.");
    }
    public static void Run(string inputPath, bool submit)
    {
        string certificate = File.ReadAllText(Required("ZATCA_CERT_PATH"));
        string key = File.ReadAllText(Required("ZATCA_KEY_PATH"));
        string token = submit ? Required("ZATCA_TOKEN") : "";
        string secret = submit ? Required("ZATCA_SECRET") : "";
        using var input = JsonDocument.Parse(File.ReadAllText(inputPath));
        var data = input.RootElement;
        var seller = data.GetProperty("seller");
        string Field(JsonElement parent, string name) => parent.GetProperty(name).GetString()
            ?? throw new InvalidOperationException($"Missing input field: {name}");
        string uuid = Guid.NewGuid().ToString();
        string output = Path.Combine("runs", uuid);
        if (Directory.Exists(output)) throw new IOException("Output directory already exists");
        Directory.CreateDirectory(output);
        Console.WriteLine($"Artifacts: {output}");
        var state = new Dictionary<string, object> {
            ["uuid"] = uuid, ["invoice_id"] = Field(data, "invoice_id"),
            ["stage"] = "create", ["submitted"] = false,
        };
        Save(Path.Combine(output, "state.json"), state);
        try
        {
            using var builder = InvoiceBuilder.New(0, 1, null, null, null, null);
            builder.SetId(Field(data, "invoice_id"));
            builder.SetUuid(uuid);
            builder.SetIssueDatetime(DateTime.UtcNow.ToString("yyyy-MM-dd'T'HH:mm:ss'Z'", CultureInfo.InvariantCulture));
            builder.SetCurrency("SAR");
            builder.SetInvoiceCounter(ulong.Parse(Field(data, "invoice_counter")));
            builder.SetPreviousInvoiceHash(Field(data, "previous_invoice_hash"));
            builder.SetPaymentMeansCode("10");
            builder.SetVatCategory(1);
            using var address = Address.New("SA", Field(seller, "city"), Field(seller, "street"),
                Field(seller, "building_number"), Field(seller, "postal_code"), null,
                Field(seller, "additional_number"), Field(seller, "district"));
            builder.SetSeller(Field(seller, "name"), address, Field(seller, "vat_id"), Field(seller, "crn"), "CRN");
            builder.AddLineItem("دفتر", "2", "PCE", "50", "15", 1);
            using var invoice = builder.Build();
            File.WriteAllText(Path.Combine(output, "invoice.xml"), invoice.Xml());
            Console.WriteLine("Created: SAR 100 net + SAR 15 VAT = SAR 115 payable");
            state["stage"] = "sign";
            using var signer = Signer.FromPem(certificate, key);
            using var signed = signer.Sign(invoice);
            string signedXml = signed.Xml();
            File.WriteAllText(Path.Combine(output, "signed.xml"), signedXml);
            state["invoice_hash"] = signed.InvoiceHash();
            state["stage"] = "validate";
            using var config = Config.New(Sandbox);
            string reportJson = Xml.ValidateZatca(config, signedXml, JsonSerializer.Serialize(new {
                previous_invoice_hash = Field(data, "previous_invoice_hash"),
            }));
            File.WriteAllText(Path.Combine(output, "validation.json"), reportJson);
            using var report = JsonDocument.Parse(reportJson);
            if (!report.RootElement.GetProperty("is_valid").GetBoolean())
                throw new InvalidOperationException("Local validation failed; inspect validation.json. Nothing was submitted.");
            Console.WriteLine("Local validation passed; inspect validation.json for warnings");
            if (!submit)
            {
                state["stage"] = "local_only";
                Console.WriteLine("Local run complete. No reporting request sent. Use --submit with sandbox credentials.");
                return;
            }
            state["stage"] = "report";
            state["submitted"] = true; // Attempted; a failed connection can leave the outcome unknown.
            Save(Path.Combine(output, "state.json"), state);
            using var credentials = CsidProduction.Create(Sandbox, null, token, secret);
            using var client = ZatcaClient.Create(config);
            using var response = client.ReportSimplifiedInvoice(signed, credentials, false, "en");
            HandleResponse(response, output);
            state["stage"] = "accepted";
        }
        catch (Exception error)
        {
            state["error_type"] = error.GetType().Name;
            if (error is BindingErrorException binding)
            {
                using var details = JsonDocument.Parse(binding.Inner.DetailsJson());
                Save(Path.Combine(output, "error.json"), new { code = binding.Inner.Code(), details = details.RootElement });
            }
            throw;
        }
        finally { Save(Path.Combine(output, "state.json"), state); }
    }
    public static int Main(string[] args)
    {
        try
        {
            string input = "input.json";
            bool submit = false;
            for (int i = 0; i < args.Length; i++)
            {
                if (args[i] == "--submit") submit = true;
                else if (args[i] == "--input" && i + 1 < args.Length) input = args[++i];
                else throw new ArgumentException("Usage: dotnet run -- [--input input.json] [--submit]");
            }
            Run(input, submit);
            return 0;
        }
        catch (Exception error)
        {
            Console.Error.WriteLine($"Stopped: {error.Message}");
            Console.Error.WriteLine("No automatic retry. Inspect saved artifacts before another run.");
            return 1;
        }
    }
}
