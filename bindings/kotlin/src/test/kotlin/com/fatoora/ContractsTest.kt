package com.fatoora

import java.io.File
import java.nio.charset.CharacterCodingException
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.test.*
import kotlinx.coroutines.*

class ContractsTest {
    @Test fun stringsOptionsAndErrors() {
        Address.create("SA", "الرياض", "Street", "1234", "12345", null, "", "حي العليا").use { a ->
            assertEquals("الرياض", a.city())
            assertNull(a.additionalStreet())
            a.additionalNumber()!!.use { assertEquals("", it.value()) }
            a.district()!!.use { assertEquals("حي العليا", it.value()) }
        }
        val error = assertFailsWith<BindingException> { Address.create("XX", "a", "b", "1", "1") }
        assertEquals(1, error.code)
        assertTrue(error.message.contains("Invalid country"))
        assertTrue(error.detailsJson.startsWith("{"))
        assertFailsWith<CharacterCodingException> { Address.create("SA", "\uD800", "b", "1", "1") }
        assertFailsWith<BindingException> { Address.create("SA", "bad\u0000city", "b", "1", "1") }
    }

    @Test fun unsignedBoundariesRemainJavaCallable() {
        InvoiceBuilder.create(InvoiceTypeKind.Tax, InvoiceSubType.Standard).use { builder ->
            assertFailsWith<IllegalArgumentException> { builder.setFlags(-1) }
            assertFailsWith<IllegalArgumentException> { builder.setFlags(256) }
            builder.setFlags(InvoiceFlag.ThirdParty.bit() or InvoiceFlag.Export.bit())
            val maximum = java.math.BigInteger("18446744073709551615")
            builder.setInvoiceCounter(maximum)
            assertFailsWith<IllegalArgumentException> { builder.setInvoiceCounter(maximum.add(java.math.BigInteger.ONE)) }
            assertFailsWith<IllegalArgumentException> { builder.setInvoiceCounter(java.math.BigInteger.valueOf(-1)) }
        }
    }

    @Test fun zatcaValidationReturnsTypedReport() {
        Xml.validateZatca("<wrong/>", """{"evaluated_at":"2026-09-23T12:00:00+03:00"}""").use { report ->
            assertFalse(report.isValid())
            assertTrue(report.hasErrors())
            assertEquals("2026-09-23T12:00:00+03:00", report.evaluatedAt())
            assertEquals(java.math.BigInteger.valueOf(6), report.stagesLen())
            report.stage(java.math.BigInteger.ZERO).use { stage ->
                assertEquals(ZatcaStage.Xsd, stage.stage())
                assertEquals(ZatcaStageStatus.Completed, stage.status())
                stage.finding(java.math.BigInteger.ZERO).use { finding ->
                    assertEquals("XSD_INVALID", finding.code())
                    assertEquals(Severity.Error, finding.severity())
                }
            }
            assertTrue(report.toJson().contains("\"is_valid\":false"))
        }
        ComplianceCsid.create(Environment.Simulation, null, "token", "secret").use { assertNull(it.requestId()) }
    }

    @Test fun keyBytesAndClose() {
        val key = SigningKey.generate()
        val pem = key.toPem()
        key.toDer().use { bytes ->
            val copy = bytes.asSlice()
            bytes.close()
            SigningKey.fromDer(copy).use { assertEquals(pem, it.toPem()) }
            assertFailsWith<IllegalStateException> { bytes.asSlice() }
        }
        key.close(); key.close()
        assertFailsWith<IllegalStateException> { key.toPem() }
    }

    @Test fun consumptionDoesNotBecomeUseAfterFree() {
        InvoiceBuilder.create(InvoiceTypeKind.Tax, InvoiceSubType.Standard).use { b ->
            assertFailsWith<BindingException> { b.build() }
            assertFailsWith<BindingException> { b.setId("again") }
        }
    }

    @Test fun closeWaitsForAnActiveNativeRead() {
        val key = SigningKey.generate()
        val entered = CountDownLatch(1)
        val release = CountDownLatch(1)
        val closing = CountDownLatch(1)
        val pool = Executors.newFixedThreadPool(2)
        try {
            val read = pool.submit<String> { nativeCall(listOf(key)) {
                entered.countDown(); check(release.await(5, TimeUnit.SECONDS)); key.toPem()
            } }
            assertTrue(entered.await(5, TimeUnit.SECONDS))
            val close = pool.submit { closing.countDown(); key.close() }
            assertTrue(closing.await(5, TimeUnit.SECONDS))
            assertFalse(close.isDone)
            release.countDown()
            assertTrue(read.get(5, TimeUnit.SECONDS).contains("PRIVATE KEY"))
            close.get(5, TimeUnit.SECONDS)
            assertFailsWith<IllegalStateException> { key.toPem() }
        } finally { release.countDown(); pool.shutdownNow(); key.close() }
    }

    @Test fun reverseArgumentOrdersDoNotDeadlock() {
        SigningKey.generate().use { a -> SigningKey.generate().use { b ->
            val pool = Executors.newFixedThreadPool(2)
            try {
                val first = pool.submit { repeat(100) { nativeCall(listOf(a,b)) { a.toPem(); b.toPem() } } }
                val second = pool.submit { repeat(100) { nativeCall(listOf(b,a)) { b.toPem(); a.toPem() } } }
                first.get(10, TimeUnit.SECONDS); second.get(10, TimeUnit.SECONDS)
            } finally { pool.shutdownNow() }
        } }
    }

    @Test fun cancellationClosesAnUndeliveredOwner() = runBlocking {
        val entered = CountDownLatch(1)
        val release = CountDownLatch(1)
        val destroyed = AtomicBoolean()
        val worker = launch { nativeIO {
            entered.countDown(); check(release.await(5, TimeUnit.SECONDS))
            object : NativeResource() { init { attach { destroyed.set(true) } } }
        } }
        withContext(Dispatchers.IO) { check(entered.await(5, TimeUnit.SECONDS)) }
        worker.cancel(); release.countDown(); worker.join()
        assertTrue(destroyed.get())
    }

    @Test fun sdkHashSigningAndPreparedRequests() {
        val root = File(System.getProperty("fatoora.fixtures"))
        val xml = File(root, "cases/simplified-invoice/input.xml").readText()
        assertEquals("aqY8frWKpauDQqXCsdne5hYwMQCo1DUUNKLfSAtT2mg=", Xml.hash(xml))
        InvoiceSigner.fromDer(File(root,"credentials/certificate.der").readBytes(), File(root,"credentials/private-key.der").readBytes()).use { signer ->
            val signed = signer.signXml(xml)
            Config.create(Environment.NonProduction).use { config -> ZatcaClient.create(config).use { client ->
                assertEquals(Environment.NonProduction, config.env())
                val credentials = ProductionCsid.create(Environment.NonProduction, null, "token", "secret")
                val invoice = SignedInvoice.fromXml(signed)
                assertEquals(Xml.hash(signed), invoice.hash())
                val request = client.prepareReportSimplifiedInvoice(invoice, credentials, ClearanceMode.Disabled)
                credentials.close(); invoice.close()
                request.use {
                    assertEquals("POST", it.method())
                    it.body().use { body -> assertTrue(String(body.asSlice()).contains("invoiceHash")) }
                    it.decode(200, """{"validationResults":{"status":"PASS"},"reportingStatus":"REPORTED"}""".toByteArray()).use { response ->
                        assertEquals(InvoiceOutcome.Accepted, response.outcome())
                        assertEquals(ReportingStatus.Reported, response.reportingStatus())
                        response.reportingStatusRaw()!!.use { raw -> assertEquals("REPORTED", raw.value()) }
                        assertNull(response.clearanceStatus())
                        response.validationResults().use { results -> assertEquals(ValidationStatus.Pass, results.status()) }
                        response.ensureAccepted()
                    }
                    assertFailsWith<BindingException> { it.decode(400, "bad response".toByteArray()) }
                }
            } }
        }
    }
}
