using Fatoora;
using System.Text.Json;

string xml = File.ReadAllText("invoice.xml");
if (args.Contains("--zatca"))
{
    string options = JsonSerializer.Serialize(new {
        previous_invoice_hash = File.ReadAllText("previous-hash.txt").Trim()
    });
    using var report = Xml.ValidateZatca(xml, options);
    Console.WriteLine(report.ToJson());
    Environment.ExitCode = report.IsValid() ? 0 : 1;
}
else
{
    Xml.Validate(xml);
    Console.WriteLine("XSD valid");
}
