using Fatoora;

using var properties = CsrProperties.FromPropertiesStr(File.ReadAllText("csr.properties"));
using var key = SigningKey.Generate();
using var csr = properties.Build(key, ZatcaEnvironment.NonProduction);
File.WriteAllText("csr.pem", csr.ToPem());
File.WriteAllText("key.pem", key.ToPem());
Console.WriteLine("Saved csr.pem and key.pem");
