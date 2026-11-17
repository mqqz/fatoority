package examples;

import com.fatoora.*;
import java.math.BigDecimal;

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

    // --8<-- [start:amounts]
    public static BigDecimal lineTotal() {
        try (InvoiceLineItem item = InvoiceLineItem.create("Item", new BigDecimal("2"), "PCE",
                 new BigDecimal("10.50"), new BigDecimal("15"), VatCategory.Standard)) {
            return item.getTotalAmount(); // getVatAmount() returns 3.15
        }
    }
    // --8<-- [end:amounts]

    // --8<-- [start:errors]
    public static String describeFailure(String xml) {
        try (SignedInvoice invoice = SignedInvoice.fromXml(xml)) {
            return "signed by " + invoice.getIssuer();
        } catch (InvalidInputException error) {
            return "bad input: " + error.getMessage();
        } catch (BindingException error) {
            return error.getKind() + ": " + error.getMessage() + " " + error.getDetailsJson();
        }
    }
    // --8<-- [end:errors]
}
