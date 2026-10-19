package com.fatoora;

import org.junit.Test;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import static org.junit.Assert.*;

public class JavaContractsTest {
    @Test public void importedSignedHashMatchesXml() throws Exception {
        Path fixtures = Paths.get(System.getProperty("fatoora.fixtures"));
        for (String kind : new String[]{"standard", "simplified"}) {
            String xml = new String(Files.readAllBytes(fixtures.resolve("cases/" + kind + "-invoice/sdk-signed.xml")), StandardCharsets.UTF_8);
            try (SignedInvoice invoice = SignedInvoice.fromXml(xml)) {
                assertEquals(Xml.hash(xml), invoice.hashBase64());
            }
        }
    }

    @Test public void nullableHttpStatusIsJavaCallable() throws Exception {
        assertEquals(Integer.class, ValidationResponse.class.getMethod("httpStatus").getReturnType());
        Path fixtures = Paths.get(System.getProperty("fatoora.fixtures"));
        String xml = new String(Files.readAllBytes(fixtures.resolve("cases/simplified-invoice/input.xml")), StandardCharsets.UTF_8);
        try (Signer signer = Signer.fromDer(
                 Files.readAllBytes(fixtures.resolve("credentials/certificate.der")),
                 Files.readAllBytes(fixtures.resolve("credentials/private-key.der")));
             SignedInvoice invoice = SignedInvoice.fromXml(signer.signXml(xml));
             Config config = Config.create(0);
             ZatcaClient client = ZatcaClient.create(config);
             CsidProduction credentials = CsidProduction.create(0, null, "token", "secret");
             InvoiceRequest request = client.prepareReportSimplifiedInvoice(invoice, credentials, false);
             ValidationResponse response = request.decode(200,
                 "{\"validationResults\":{\"status\":\"PASS\"},\"reportingStatus\":\"REPORTED\"}".getBytes(StandardCharsets.UTF_8))) {
            Integer status = response.httpStatus();
            assertEquals(Integer.valueOf(200), status);
        }
    }

    @Test public void javaUsesFactoriesOverloadsAndExceptions() {
        try (SigningKey key = SigningKey.generate();
             SigningKey restored = SigningKey.fromPem(key.toPem());
             Address address = Address.create("SA", "الرياض", "Street", "1234", "12345")) {
            assertEquals(key.toPem(), restored.toPem());
            assertEquals("الرياض", address.city());
            assertNull(address.additionalStreet());
        }
        try { Config.create(99); fail("Expected BindingException"); }
        catch (BindingException e) { assertEquals(1, e.getCode()); assertNotNull(e.getMessage()); }
    }
}
