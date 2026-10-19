package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceDataLib: Library {
    fun fatoora_InvoiceData_destroy(handle: Pointer)
    fun fatoora_InvoiceData_issue_time(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_issue_date(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_payment_means_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceData_payment_means(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_InvoiceData_purchase_order_id(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_contract_id(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_payment_instruction(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_actual_delivery_date(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_latest_delivery_date(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_tax_currency(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_tax_currency_amount(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_adjustments_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceData_adjustment(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_InvoiceData_id(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_uuid(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_previous_invoice_hash(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_payment_means_code(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_currency(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_issue_datetime(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_invoice_level_charge(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_invoice_level_discount(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceData_allowance_reason(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_invoice_counter(handle: Pointer): FFIUint64
    fun fatoora_InvoiceData_vat_category(handle: Pointer): FFIUint8
    fun fatoora_InvoiceData_flags_raw(handle: Pointer): FFIUint8
    fun fatoora_InvoiceData_invoice_type_kind(handle: Pointer): FFIUint8
    fun fatoora_InvoiceData_invoice_sub_type(handle: Pointer): FFIUint8
    fun fatoora_InvoiceData_seller(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_buyer(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_note(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_line_items_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceData_line_item(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_InvoiceData_original_invoice_ref(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_original_invoice_reason(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceData_notes_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceData_note_at(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_InvoiceData_billing_references_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceData_billing_reference(handle: Pointer, index: FFISizet): ResultPointerPointer
}

class InvoiceData internal constructor (
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
            lib.fatoora_InvoiceData_destroy(handle)
    }

    companion object {
        internal val libClass: Class<InvoiceDataLib> = InvoiceDataLib::class.java
        internal val lib: InvoiceDataLib = Native.load("fatoora_ffi", libClass)
    }

    fun issueTime(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_issue_time(handle, write);
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

    fun issueDate(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_issue_date(handle, write);
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

    fun paymentMeansLen(): ULong {

        val returnVal = lib.fatoora_InvoiceData_payment_means_len(handle);
        return (returnVal.toULong())
    }

    fun paymentMeans(index: ULong): Result<PaymentMeans> {

        val returnVal = lib.fatoora_InvoiceData_payment_means(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = PaymentMeans(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun purchaseOrderId(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_purchase_order_id(handle);
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

    fun contractId(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_contract_id(handle);
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

    fun paymentInstruction(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_payment_instruction(handle);
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

    fun actualDeliveryDate(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_actual_delivery_date(handle);
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

    fun latestDeliveryDate(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_latest_delivery_date(handle);
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

    fun taxCurrency(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_tax_currency(handle);
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

    fun taxCurrencyAmount(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_tax_currency_amount(handle);
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

    fun adjustmentsLen(): ULong {

        val returnVal = lib.fatoora_InvoiceData_adjustments_len(handle);
        return (returnVal.toULong())
    }

    fun adjustment(index: ULong): Result<AllowanceCharge> {

        val returnVal = lib.fatoora_InvoiceData_adjustment(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = AllowanceCharge(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun id(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_id(handle, write);
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

    fun uuid(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_uuid(handle, write);
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

    fun previousInvoiceHash(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_previous_invoice_hash(handle, write);
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

    fun paymentMeansCode(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_payment_means_code(handle, write);
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

    fun currency(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_currency(handle, write);
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

    fun issueDatetime(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_issue_datetime(handle, write);
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

    fun invoiceLevelCharge(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_invoice_level_charge(handle, write);
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

    fun invoiceLevelDiscount(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceData_invoice_level_discount(handle, write);
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

    fun allowanceReason(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_allowance_reason(handle);
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

    fun invoiceCounter(): ULong {

        val returnVal = lib.fatoora_InvoiceData_invoice_counter(handle);
        return (returnVal.toULong())
    }

    fun vatCategory(): UByte {

        val returnVal = lib.fatoora_InvoiceData_vat_category(handle);
        return (returnVal.toUByte())
    }

    fun flagsRaw(): UByte {

        val returnVal = lib.fatoora_InvoiceData_flags_raw(handle);
        return (returnVal.toUByte())
    }

    fun invoiceTypeKind(): UByte {

        val returnVal = lib.fatoora_InvoiceData_invoice_type_kind(handle);
        return (returnVal.toUByte())
    }

    fun invoiceSubType(): UByte {

        val returnVal = lib.fatoora_InvoiceData_invoice_sub_type(handle);
        return (returnVal.toUByte())
    }

    fun seller(): Result<Party> {

        val returnVal = lib.fatoora_InvoiceData_seller(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = Party(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun buyer(): Result<Party?> {

        val returnVal = lib.fatoora_InvoiceData_buyer(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Party(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun note(): Result<InvoiceNote?> {

        val returnVal = lib.fatoora_InvoiceData_note(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = InvoiceNote(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun lineItemsLen(): ULong {

        val returnVal = lib.fatoora_InvoiceData_line_items_len(handle);
        return (returnVal.toULong())
    }

    fun lineItem(index: ULong): Result<InvoiceLineItem> {

        val returnVal = lib.fatoora_InvoiceData_line_item(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = InvoiceLineItem(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun originalInvoiceRef(): Result<OriginalInvoiceRef?> {

        val returnVal = lib.fatoora_InvoiceData_original_invoice_ref(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = OriginalInvoiceRef(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun originalInvoiceReason(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceData_original_invoice_reason(handle);
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

    fun notesLen(): ULong {

        val returnVal = lib.fatoora_InvoiceData_notes_len(handle);
        return (returnVal.toULong())
    }

    fun noteAt(index: ULong): Result<InvoiceNote> {

        val returnVal = lib.fatoora_InvoiceData_note_at(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = InvoiceNote(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun billingReferencesLen(): ULong {

        val returnVal = lib.fatoora_InvoiceData_billing_references_len(handle);
        return (returnVal.toULong())
    }

    fun billingReference(index: ULong): Result<OriginalInvoiceRef> {

        val returnVal = lib.fatoora_InvoiceData_billing_reference(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = OriginalInvoiceRef(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
