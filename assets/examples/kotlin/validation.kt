import com.fatoora.*
import java.io.File
import kotlinx.serialization.json.*

fun main(args: Array<String>) {
    val xml = File("invoice.xml").readText()
    if (args.contains("--zatca")) {
        val hash = File("previous-hash.txt").readText().trim()
        val options = buildJsonObject { put("previous_invoice_hash", JsonPrimitive(hash)) }.toString()
        Xml.validateZatca(xml, options).use { report ->
            println(report.toJson())
            if (!report.isValid) kotlin.system.exitProcess(1)
        }
    } else {
        Xml.validate(xml)
        println("XSD valid")
    }
}
