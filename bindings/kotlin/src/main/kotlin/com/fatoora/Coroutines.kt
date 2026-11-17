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

private fun clearanceMode(enabled: Boolean): ClearanceMode =
    if (enabled) ClearanceMode.Enabled else ClearanceMode.Disabled

suspend fun ZatcaClient.postCsrForCcsidAsync(csr: Csr, otp: String): CsidCompliance =
    nativeIO { postCsrForCcsid(csr, otp) }
suspend fun ZatcaClient.postCcsidForPcsidAsync(credentials: CsidCompliance): CsidProduction =
    nativeIO { postCcsidForPcsid(credentials) }
suspend fun ZatcaClient.renewCsidAsync(credentials: CsidProduction, csr: Csr, otp: String,
                                     acceptLanguage: String? = null): CsidProduction =
    nativeIO { renewCsid(credentials, csr, otp, acceptLanguage) }
suspend fun ZatcaClient.checkInvoiceComplianceAsync(invoice: SignedInvoice, credentials: CsidCompliance): ValidationResponse =
    nativeIO { checkInvoiceCompliance(invoice, credentials) }
suspend fun ZatcaClient.reportSimplifiedInvoiceAsync(invoice: SignedInvoice, credentials: CsidProduction,
                                                    clearanceStatus: Boolean, acceptLanguage: String? = null): ValidationResponse =
    nativeIO { reportSimplifiedInvoice(invoice, credentials, clearanceMode(clearanceStatus), acceptLanguage) }
suspend fun ZatcaClient.clearStandardInvoiceAsync(invoice: SignedInvoice, credentials: CsidProduction,
                                                clearanceStatus: Boolean, acceptLanguage: String? = null): ValidationResponse =
    nativeIO { clearStandardInvoice(invoice, credentials, clearanceMode(clearanceStatus), acceptLanguage) }
