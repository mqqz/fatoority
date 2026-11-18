package com.fatoora

import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.util.concurrent.atomic.AtomicReference

/** Offloads a blocking call. Cancellation cannot interrupt an in-flight native call. */
internal suspend fun <T : NativeResource> nativeIO(block: () -> T): T {
    val returned = AtomicReference<T>()
    try {
        return withContext(Dispatchers.IO) { block().also { returned.set(it) } }
    } catch (cancelled: CancellationException) {
        // withContext can discard a completed result during dispatch back to the caller.
        returned.get()?.close()
        throw cancelled
    }
}

suspend fun ZatcaClient.requestComplianceCsidAsync(csr: Csr, otp: String): ComplianceCsid =
    nativeIO { requestComplianceCsid(csr, otp) }
suspend fun ZatcaClient.requestProductionCsidAsync(credentials: ComplianceCsid): ProductionCsid =
    nativeIO { requestProductionCsid(credentials) }
suspend fun ZatcaClient.renewCsidAsync(credentials: ProductionCsid, csr: Csr, otp: String,
                                     acceptLanguage: String? = null): ProductionCsid =
    nativeIO { renewCsid(credentials, csr, otp, acceptLanguage) }
suspend fun ZatcaClient.checkInvoiceComplianceAsync(invoice: SignedInvoice, credentials: ComplianceCsid): ValidationResponse =
    nativeIO { checkInvoiceCompliance(invoice, credentials) }
suspend fun ZatcaClient.reportSimplifiedInvoiceAsync(invoice: SignedInvoice, credentials: ProductionCsid,
                                                    clearanceMode: ClearanceMode, acceptLanguage: String? = null): ValidationResponse =
    nativeIO { reportSimplifiedInvoice(invoice, credentials, clearanceMode, acceptLanguage) }
suspend fun ZatcaClient.clearStandardInvoiceAsync(invoice: SignedInvoice, credentials: ProductionCsid,
                                                clearanceMode: ClearanceMode, acceptLanguage: String? = null): ValidationResponse =
    nativeIO { clearStandardInvoice(invoice, credentials, clearanceMode, acceptLanguage) }
