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
    fun fatoora_ValidationResponse_cleared_invoice_xml_present(handle: Pointer): Byte
    fun fatoora_ValidationResponse_cleared_invoice_xml(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ValidationResponse_validation_results(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResponse_cleared_invoice_base64(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationResponse_reporting_status(handle: Pointer): OptionInt
    fun fatoora_ValidationResponse_reporting_status_raw(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationResponse_clearance_status(handle: Pointer): OptionInt
    fun fatoora_ValidationResponse_clearance_status_raw(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationResponse_qr_seller_status(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ValidationResponse_qr_buyer_status(handle: Pointer, write: Pointer): OptionUnit
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

    /**
     * Whether the response carries a cleared invoice.
     */
    fun clearedInvoiceXmlPresent(): Boolean {

        val returnVal = lib.fatoora_ValidationResponse_cleared_invoice_xml_present(handle);
        return (returnVal > 0)
    }

    /**
     * The decoded cleared invoice, or null when the response has none.
     * Fails when the content is not base64-encoded UTF-8 text without NUL.
     */
    fun clearedInvoiceXml(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationResponse_cleared_invoice_xml(handle, write);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {

            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            DW.lib.diplomat_buffer_write_destroy(write)
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

    fun clearedInvoiceBase64(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationResponse_cleared_invoice_base64(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

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
    fun reportingStatusRaw(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationResponse_reporting_status_raw(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

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
    fun clearanceStatusRaw(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationResponse_clearance_status_raw(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    fun qrSellerStatus(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationResponse_qr_seller_status(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    fun qrBuyerStatus(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ValidationResponse_qr_buyer_status(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

}
