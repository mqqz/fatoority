package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ValidationResponseLib: Library {
    fun fatoora_ValidationResponse_destroy(handle: Pointer)
    fun fatoora_ValidationResponse_http_status(handle: Pointer): OptionFFIUint16
    fun fatoora_ValidationResponse_outcome(handle: Pointer): Int
    fun fatoora_ValidationResponse_ensure_accepted(handle: Pointer): ResultUnitPointer
    fun fatoora_ValidationResponse_cleared_invoice_xml(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResponse_validation_results(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResponse_cleared_invoice_base64(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResponse_reporting_status(handle: Pointer): OptionInt
    fun fatoora_ValidationResponse_reporting_status_raw(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResponse_clearance_status(handle: Pointer): OptionInt
    fun fatoora_ValidationResponse_clearance_status_raw(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResponse_qr_seller_status(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResponse_qr_buyer_status(handle: Pointer): ResultPointerPointer
}

class ValidationResponse internal constructor (
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
            lib.fatoora_ValidationResponse_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ValidationResponseLib> = ValidationResponseLib::class.java
        internal val lib: ValidationResponseLib = Native.load("fatoora_ffi", libClass)
    }

    fun httpStatus(): UShort? {

        val returnVal = lib.fatoora_ValidationResponse_http_status(handle);
        return returnVal.option()?.toUShort()
    }

    fun outcome(): InvoiceOutcome {

        val returnVal = lib.fatoora_ValidationResponse_outcome(handle);
        return (InvoiceOutcome.fromNative(returnVal))
    }

    fun ensureAccepted(): Result<Unit> {

        val returnVal = lib.fatoora_ValidationResponse_ensure_accepted(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            return Unit.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun clearedInvoiceXml(): Result<Text?> {

        val returnVal = lib.fatoora_ValidationResponse_cleared_invoice_xml(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun validationResults(): Result<ValidationResults> {

        val returnVal = lib.fatoora_ValidationResponse_validation_results(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ValidationResults(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun clearedInvoiceBase64(): Result<Text?> {

        val returnVal = lib.fatoora_ValidationResponse_cleared_invoice_base64(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    /**
     * The parsed `reportingStatus`, or null when the response has none.
     */
    fun reportingStatus(): ReportingStatus? {

        val returnVal = lib.fatoora_ValidationResponse_reporting_status(handle);

        val intermediateOption = returnVal.option() ?: return null
        return ReportingStatus.fromNative(intermediateOption)
    }

    /**
     * The `reportingStatus` exactly as ZATCA sent it, or null when absent.
     */
    fun reportingStatusRaw(): Result<Text?> {

        val returnVal = lib.fatoora_ValidationResponse_reporting_status_raw(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    /**
     * The parsed `clearanceStatus`, or null when the response has none.
     */
    fun clearanceStatus(): ClearanceStatus? {

        val returnVal = lib.fatoora_ValidationResponse_clearance_status(handle);

        val intermediateOption = returnVal.option() ?: return null
        return ClearanceStatus.fromNative(intermediateOption)
    }

    /**
     * The `clearanceStatus` exactly as ZATCA sent it, or null when absent.
     */
    fun clearanceStatusRaw(): Result<Text?> {

        val returnVal = lib.fatoora_ValidationResponse_clearance_status_raw(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun qrSellerStatus(): Result<Text?> {

        val returnVal = lib.fatoora_ValidationResponse_qr_seller_status(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun qrBuyerStatus(): Result<Text?> {

        val returnVal = lib.fatoora_ValidationResponse_qr_buyer_status(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
