package com.fatoora;

import androidx.test.ext.junit.runners.AndroidJUnit4;
import androidx.test.platform.app.InstrumentationRegistry;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.math.BigDecimal;
import java.nio.charset.StandardCharsets;
import org.junit.Test;
import org.junit.runner.RunWith;
import static org.junit.Assert.*;

@RunWith(AndroidJUnit4.class)
public class AndroidContractsTest {
    @Test public void javaApiAndNativeLoading() {
        try (SigningKey key = SigningKey.generate();
             SigningKey restored = SigningKey.fromPem(key.toPem());
             Address address = Address.create("SA", "الرياض", "Street", "1234", "12345")) {
            assertEquals(key.toPem(), restored.toPem());
            assertEquals("الرياض", address.getCity());
            assertNull(address.getAdditionalStreet());
        }
        try { Address.create("XX", "a", "b", "1", "1"); fail("invalid country accepted"); }
        catch (InvalidInputException expected) { assertEquals(ErrorKind.InvalidInput, expected.getKind()); }
        try (InvoiceLineItem item = InvoiceLineItem.create("Item", new BigDecimal("2"), "PCE",
                 new BigDecimal("10.50"), new BigDecimal("15"), VatCategory.Standard)) {
            assertEquals(0, new BigDecimal("21").compareTo(item.getTotalAmount()));
        }
    }
    @Test public void clientInitializesTlsAndPreparesRequests() {
        try (Config config = Config.create(Environment.NonProduction);
             ZatcaClient client = ZatcaClient.create(config);
             ProductionCsid credentials = ProductionCsid.create(Environment.NonProduction, null, "token", "secret")) {
            assertNotNull(client);
            assertNotNull(credentials);
        }
    }
    @Test public void signedXmlMatchesSdk() throws Exception {
        try (InputStream stream = InstrumentationRegistry.getInstrumentation().getContext()
                .getAssets().open("cases/simplified-invoice/input.xml");
             ByteArrayOutputStream bytes = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[8192];
            int count;
            while ((count = stream.read(buffer)) != -1) bytes.write(buffer, 0, count);
            assertEquals("aqY8frWKpauDQqXCsdne5hYwMQCo1DUUNKLfSAtT2mg=",
                Xml.hash(new String(bytes.toByteArray(), StandardCharsets.UTF_8)));
        }
    }
}
