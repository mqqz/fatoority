import com.fatoora.*;
import java.nio.file.*;
import java.nio.charset.StandardCharsets;

public class Guide {
    static String read(String path) throws java.io.IOException {
        return new String(Files.readAllBytes(Paths.get(path)), StandardCharsets.UTF_8);
    }
    public static void main(String[] args) throws Exception {
        String xml = read("invoice.xml");
        if (java.util.Arrays.asList(args).contains("--zatca")) {
            com.google.gson.JsonObject options = new com.google.gson.JsonObject();
            options.addProperty("previous_invoice_hash", read("previous-hash.txt").trim());
            try (ZatcaValidationReport report = Xml.validateZatca(xml, options.toString())) {
                System.out.println(report.toJson());
                if (!report.isValid()) System.exit(1);
            }
        } else {
            Xml.validate(xml);
            System.out.println("XSD valid");
        }
    }
}
