package examples;

import com.fatoora.*;

public final class JavaExamples {
    private JavaExamples() {}

    // --8<-- [start:signing]
    public static String signInvoice(String xml, String certificatePem, String keyPem) {
        try (Signer signer = Signer.fromPem(certificatePem, keyPem);
             FinalizedInvoice invoice = FinalizedInvoice.fromXml(xml);
             SignedInvoice signed = signer.sign(invoice)) {
            return signed.intoXml();
        }
    }
    // --8<-- [end:signing]

    // --8<-- [start:validation]
    public static String validateInvoice(String xml) {
        try (Config config = Config.create(Environment.NonProduction)) {
            return Xml.validateZatca(config, xml);
        }
    }
    // --8<-- [end:validation]
}
