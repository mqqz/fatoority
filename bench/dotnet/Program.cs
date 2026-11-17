using System;
using System.IO;
using System.Linq;
using System.Xml;
using System.Text.Json;
using System.Diagnostics;
using System.Collections.Generic;
using ZATCA.EInvoice.SDK;
using Fatoora;
class Program
{
    static XmlDocument Doc(string xml)
    {
        var d = new XmlDocument { PreserveWhitespace = true };
        d.LoadXml(xml);
        return d;
    }
    static void Cold(string impl, string op)
    {
        var root = Environment.GetEnvironmentVariable("FATOORA_BENCH_FIXTURES") ?? throw new InvalidOperationException("Run through bench/dotnet/run.py to set the fixture directory.");
        var xml = File.ReadAllText(root + "/cases/simplified-invoice/" + (op == "validate" ? "sdk-signed.xml" : "input.xml"));
        var cert = File.ReadAllBytes(root + "/credentials/certificate.der");
        var key = File.ReadAllBytes(root + "/credentials/private-key.der");
        var pih = Doc(xml).SelectSingleNode("//*[local-name()='AdditionalDocumentReference'][*[local-name()='ID']='PIH']//*[local-name()='EmbeddedDocumentBinaryObject']")!.InnerText;
        if (impl == "ours")
        {
            if (op == "hash")
                Console.WriteLine(Xml.Hash(xml));
            else if (op == "sign")
            {
                using var s = InvoiceSigner.FromDer(cert, key);
                Console.WriteLine(s.SignXml(xml).Length);
            }
            else
            {
                using var r = Xml.ValidateZatca(xml, JsonSerializer.Serialize(new
                {
                    previous_invoice_hash = pih
                }));
                if (!r.IsValid)
                    throw new Exception("invalid");
                Console.WriteLine("valid");
            }
        }
        else
        {
            if (op == "hash")
            {
                var r = new EInvoiceHashGenerator().GenerateEInvoiceHashing(Doc(xml));
                if (!r.IsValid)
                    throw new Exception(r.ErrorMessage);
                Console.WriteLine(r.Hash);
            }
            else if (op == "sign")
            {
                using var ec = System.Security.Cryptography.ECDsa.Create();
                ec.ImportPkcs8PrivateKey(key, out _);
                var r = new EInvoiceSigner().SignDocument(Doc(xml), Convert.ToBase64String(cert), Convert.ToBase64String(ec.ExportECPrivateKey()));
                if (!r.IsValid)
                    throw new Exception(r.ErrorMessage);
                Console.WriteLine(r.SignedEInvoice.OuterXml.Length);
            }
            else
            {
                var r = new EInvoiceValidator().ValidateEInvoice(Doc(xml), Convert.ToBase64String(cert), pih);
                if (!r.IsValid)
                    throw new Exception(JsonSerializer.Serialize(r));
                Console.WriteLine("valid");
            }
        }
    }
    static void Main(string[] args)
    {
        if (args[0] == "cold")
        {
            Cold(args[1], args[2]);
            return;
        }
        var root = Environment.GetEnvironmentVariable("FATOORA_BENCH_FIXTURES") ?? throw new InvalidOperationException("Run through bench/dotnet/run.py to set the fixture directory.");
        var kind = args[0];
        var impl = args[1];
        var op = args[2];
        var xml = File.ReadAllText($"{root}/cases/{kind}-invoice/input.xml");
        var certDer = File.ReadAllBytes(root + "/credentials/certificate.der");
        var keyDer = File.ReadAllBytes(root + "/credentials/private-key.der");
        var cert = Convert.ToBase64String(certDer);
        using var ec = System.Security.Cryptography.ECDsa.Create();
        ec.ImportPkcs8PrivateKey(keyDer, out _);
        var key = Convert.ToBase64String(ec.ExportECPrivateKey());
        using var expectedDocument = JsonDocument.Parse(File.ReadAllText($"{root}/cases/{kind}-invoice/expected.json"));
        var expected = expectedDocument.RootElement.GetProperty("hash").GetString();
        using var signer = InvoiceSigner.FromDer(certDer, keyDer);
        var oursSigned = signer.SignXml(xml);
        var signed = File.ReadAllText($"{root}/cases/{kind}-invoice/sdk-signed.xml");
        var pih = Doc(xml).SelectSingleNode("//*[local-name()='AdditionalDocumentReference'][*[local-name()='ID']='PIH']//*[local-name()='EmbeddedDocumentBinaryObject']")!.InnerText;
        var options = JsonSerializer.Serialize(new
        {
            previous_invoice_hash = pih
        });
        var oh = new EInvoiceHashGenerator();
        var os = new EInvoiceSigner();
        var ov = new EInvoiceValidator();
        string Invoke()
        {
            if (impl == "ours")
            {
                if (op == "hash")
                {
                    var h = Xml.Hash(xml);
                    if (h != expected)
                        throw new Exception("hash mismatch");
                    return h;
                }
                if (op == "sign")
                {
                    var s = signer.SignXml(xml);
                    if (s.Length < xml.Length)
                        throw new Exception("short signature output");
                    return "signed";
                }
                using var r = Xml.ValidateZatca(signed, options);
                if (!r.IsValid)
                    throw new Exception("our validation failed: " + r.ToJson());
                return "valid";
            }
            if (op == "hash")
            {
                var r = oh.GenerateEInvoiceHashing(Doc(xml));
                if (!r.IsValid || r.Hash != expected)
                    throw new Exception("official hash " + r.ErrorMessage + " " + r.Exception);
                return r.Hash;
            }
            if (op == "sign")
            {
                var r = os.SignDocument(Doc(xml), cert, key);
                if (!r.IsValid)
                    throw new Exception("official sign " + r.ErrorMessage + " " + string.Join(";", r.Steps.Select(x => x.Exception?.ToString())));
                return "signed";
            }
            var v = ov.ValidateEInvoice(Doc(signed), cert, pih);
            if (!v.IsValid)
                throw new Exception("official validation failed: " + JsonSerializer.Serialize(v));
            return "valid";
        }
        var sw = Stopwatch.StartNew();
        var result = Invoke();
        sw.Stop();
        var first = sw.Elapsed.TotalMilliseconds;
        Console.WriteLine("RESULT " + result);
        if (op == "sign")
        {
            var osigned = os.SignDocument(Doc(xml), cert, key);
            if (!osigned.IsValid)
                throw new Exception("sign preflight");
            var sx = osigned.SignedEInvoice.OuterXml;
            if (Xml.Hash(sx) != expected || Xml.Hash(oursSigned) != expected)
                throw new Exception("signed hash differs");
            var a = ov.ValidateEInvoice(Doc(sx), cert, pih);
            var b = ov.ValidateEInvoice(Doc(oursSigned), cert, pih);
            if (!a.IsValid || !b.IsValid)
                throw new Exception("cross-validation failed");
            Console.WriteLine("PREFLIGHT both signed outputs pass official validation and preserve expected hash");
        }
        if (args.Length > 3 && args[3] == "probe")
            return;
        for (int i = 0; i < 15; i++)
            Invoke();
        sw.Restart();
        Invoke();
        sw.Stop();
        int n = Math.Clamp((int)(200 / Math.Max(sw.Elapsed.TotalMilliseconds, .01)), 1, 1000);
        var times = new List<double>();
        for (int round = 0; round < 7; round++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
            sw.Restart();
            for (int i = 0; i < n; i++)
                Invoke();
            sw.Stop();
            times.Add(sw.Elapsed.TotalMilliseconds / n);
        }
        Console.WriteLine(JsonSerializer.Serialize(new
        {
            kind,
            impl,
            op,
            first_ms = first,
            iterations = n,
            samples_ms = times,
            median_ms = times.Order().ElementAt(3)
        }));
    }
}
