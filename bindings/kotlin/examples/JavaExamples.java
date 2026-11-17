package examples;

import com.fatoora.*;

public final class JavaExamples {
    private JavaExamples() {}

    // --8<-- [start:signing]
    public static String signInvoice(String xml, String certificatePem, String keyPem) {
        try (InvoiceSigner signer = InvoiceSigner.fromPem(certificatePem, keyPem);
             FinalizedInvoice invoice = FinalizedInvoice.fromXml(xml);
             SignedInvoice signed = signer.sign(invoice)) {
            return signed.intoXml();
        }
    }
    // --8<-- [end:signing]

    // --8<-- [start:validation]
    public static boolean validateInvoice(String xml) {
        try (ZatcaValidationReport report = Xml.validateZatca(xml)) {
            return report.isValid(); // isComplete(), stage(i) and toJson() add detail
        }
    }
    // --8<-- [end:validation]
}
