package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZatcaClientLib: Library {
    fun fatoora_ZatcaClient_destroy(handle: Pointer)
    fun fatoora_ZatcaClient_create(config: Pointer): ResultPointerPointer
    fun fatoora_ZatcaClient_create_with_timeout(config: Pointer, timeoutMs: FFIUint32): ResultPointerPointer
    fun fatoora_ZatcaClient_request_compliance_csid(handle: Pointer, csr: Pointer, otp: Slice): ResultPointerPointer
    fun fatoora_ZatcaClient_request_production_csid(handle: Pointer, credentials: Pointer): ResultPointerPointer
    fun fatoora_ZatcaClient_renew_csid(handle: Pointer, credentials: Pointer, csr: Pointer, otp: Slice, acceptLanguage: OptionSlice): ResultPointerPointer
    fun fatoora_ZatcaClient_check_invoice_compliance(handle: Pointer, invoice: Pointer, credentials: Pointer): ResultPointerPointer
    fun fatoora_ZatcaClient_report_simplified_invoice(handle: Pointer, invoice: Pointer, credentials: Pointer, clearanceMode: Int, acceptLanguage: OptionSlice): ResultPointerPointer
    fun fatoora_ZatcaClient_clear_standard_invoice(handle: Pointer, invoice: Pointer, credentials: Pointer, clearanceMode: Int, acceptLanguage: OptionSlice): ResultPointerPointer
    fun fatoora_ZatcaClient_prepare_request_compliance_csid(handle: Pointer, csr: Pointer, otp: Slice): ResultPointerPointer
    fun fatoora_ZatcaClient_prepare_request_production_csid(handle: Pointer, credentials: Pointer): ResultPointerPointer
    fun fatoora_ZatcaClient_prepare_renew_csid(handle: Pointer, credentials: Pointer, csr: Pointer, otp: Slice, acceptLanguage: OptionSlice): ResultPointerPointer
    fun fatoora_ZatcaClient_prepare_check_invoice_compliance(handle: Pointer, invoice: Pointer, credentials: Pointer): ResultPointerPointer
    fun fatoora_ZatcaClient_prepare_report_simplified_invoice(handle: Pointer, invoice: Pointer, credentials: Pointer, clearanceMode: Int, acceptLanguage: OptionSlice): ResultPointerPointer
    fun fatoora_ZatcaClient_prepare_clear_standard_invoice(handle: Pointer, invoice: Pointer, credentials: Pointer, clearanceMode: Int, acceptLanguage: OptionSlice): ResultPointerPointer
}

class ZatcaClient internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    @Volatile
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerFinalizer()
        }
    }
    private fun registerFinalizer() {
        this.owned = true
    }

    @Override
    @SuppressWarnings("Finalize")
    fun finalize() {
        if (owned)
            lib.fatoora_ZatcaClient_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ZatcaClientLib> = ZatcaClientLib::class.java
        internal val lib: ZatcaClientLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun create(config: Config): Result<ZatcaClient> {

            val returnVal = lib.fatoora_ZatcaClient_create(config.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ZatcaClient(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        }
        @JvmStatic

        /**
         * Like `create`, but each request fails with a network error after
         * the given number of milliseconds. Zero is rejected.
         */
        fun createWithTimeout(config: Config, timeoutMs: UInt): Result<ZatcaClient> {

            val returnVal = lib.fatoora_ZatcaClient_create_with_timeout(config.handle, FFIUint32(timeoutMs));
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ZatcaClient(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        }
    }

    fun requestComplianceCsid(csr: Csr, otp: String): Result<ComplianceCsid> {
        val otpSliceMemory = PrimitiveArrayTools.borrowUtf8(otp)

        val returnVal = lib.fatoora_ZatcaClient_request_compliance_csid(handle, csr.handle, otpSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ComplianceCsid(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            otpSliceMemory.close()
        }
    }

    fun requestProductionCsid(credentials: ComplianceCsid): Result<ProductionCsid> {

        val returnVal = lib.fatoora_ZatcaClient_request_production_csid(handle, credentials.handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ProductionCsid(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun renewCsid(credentials: ProductionCsid, csr: Csr, otp: String, acceptLanguage: String?): Result<ProductionCsid> {
        val otpSliceMemory = PrimitiveArrayTools.borrowUtf8(otp)
        val acceptLanguageSliceMemory = acceptLanguage?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_ZatcaClient_renew_csid(handle, credentials.handle, csr.handle, otpSliceMemory.slice, acceptLanguageSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ProductionCsid(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            otpSliceMemory.close()
            acceptLanguageSliceMemory?.close()
        }
    }

    fun checkInvoiceCompliance(invoice: SignedInvoice, credentials: ComplianceCsid): Result<ValidationResponse> {

        val returnVal = lib.fatoora_ZatcaClient_check_invoice_compliance(handle, invoice.handle, credentials.handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ValidationResponse(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun reportSimplifiedInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceMode: ClearanceMode, acceptLanguage: String?): Result<ValidationResponse> {
        val acceptLanguageSliceMemory = acceptLanguage?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_ZatcaClient_report_simplified_invoice(handle, invoice.handle, credentials.handle, clearanceMode.toNative(), acceptLanguageSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ValidationResponse(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            acceptLanguageSliceMemory?.close()
        }
    }

    fun clearStandardInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceMode: ClearanceMode, acceptLanguage: String?): Result<ValidationResponse> {
        val acceptLanguageSliceMemory = acceptLanguage?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_ZatcaClient_clear_standard_invoice(handle, invoice.handle, credentials.handle, clearanceMode.toNative(), acceptLanguageSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ValidationResponse(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            acceptLanguageSliceMemory?.close()
        }
    }

    fun prepareRequestComplianceCsid(csr: Csr, otp: String): Result<ComplianceRequest> {
        val otpSliceMemory = PrimitiveArrayTools.borrowUtf8(otp)

        val returnVal = lib.fatoora_ZatcaClient_prepare_request_compliance_csid(handle, csr.handle, otpSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ComplianceRequest(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            otpSliceMemory.close()
        }
    }

    fun prepareRequestProductionCsid(credentials: ComplianceCsid): Result<ProductionRequest> {

        val returnVal = lib.fatoora_ZatcaClient_prepare_request_production_csid(handle, credentials.handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ProductionRequest(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun prepareRenewCsid(credentials: ProductionCsid, csr: Csr, otp: String, acceptLanguage: String?): Result<ProductionRequest> {
        val otpSliceMemory = PrimitiveArrayTools.borrowUtf8(otp)
        val acceptLanguageSliceMemory = acceptLanguage?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_ZatcaClient_prepare_renew_csid(handle, credentials.handle, csr.handle, otpSliceMemory.slice, acceptLanguageSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = ProductionRequest(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            otpSliceMemory.close()
            acceptLanguageSliceMemory?.close()
        }
    }

    fun prepareCheckInvoiceCompliance(invoice: SignedInvoice, credentials: ComplianceCsid): Result<InvoiceRequest> {

        val returnVal = lib.fatoora_ZatcaClient_prepare_check_invoice_compliance(handle, invoice.handle, credentials.handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = InvoiceRequest(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun prepareReportSimplifiedInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceMode: ClearanceMode, acceptLanguage: String?): Result<InvoiceRequest> {
        val acceptLanguageSliceMemory = acceptLanguage?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_ZatcaClient_prepare_report_simplified_invoice(handle, invoice.handle, credentials.handle, clearanceMode.toNative(), acceptLanguageSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = InvoiceRequest(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            acceptLanguageSliceMemory?.close()
        }
    }

    fun prepareClearStandardInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceMode: ClearanceMode, acceptLanguage: String?): Result<InvoiceRequest> {
        val acceptLanguageSliceMemory = acceptLanguage?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_ZatcaClient_prepare_clear_standard_invoice(handle, invoice.handle, credentials.handle, clearanceMode.toNative(), acceptLanguageSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = InvoiceRequest(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            acceptLanguageSliceMemory?.close()
        }
    }

}
