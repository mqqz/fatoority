package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceLineItemLib: Library {
    fun fatoora_InvoiceLineItem_destroy(handle: Pointer)
    fun fatoora_InvoiceLineItem_item_gross_price(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_item_price_discount(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_item_price_base_quantity(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_item_price_base_quantity_unit_code(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_adjustments_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceLineItem_adjustment(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_InvoiceLineItem_adjustment_is_charge(handle: Pointer, index: FFISizet): ResultBytePointer
    fun fatoora_InvoiceLineItem_item_seller_identifier(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_item_buyer_identifier(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_item_standard_identifier(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_with_document_reference(handle: Pointer, reference: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_with_tax_subtotal(handle: Pointer, subtotal: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_document_references_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceLineItem_document_reference(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_InvoiceLineItem_tax_subtotals_len(handle: Pointer): FFISizet
    fun fatoora_InvoiceLineItem_tax_subtotal(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_InvoiceLineItem_new(description: Slice, quantity: Slice, unitCode: Slice, unitPrice: Slice, rate: Slice, category: FFIUint8): ResultPointerPointer
    fun fatoora_InvoiceLineItem_with_metadata(handle: Pointer, id: OptionSlice, exemption: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_id(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_exemption(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceLineItem_description(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceLineItem_unit_code(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceLineItem_quantity(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceLineItem_unit_price(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceLineItem_total_amount(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceLineItem_vat_rate(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceLineItem_vat_amount(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceLineItem_has_vat_rate(handle: Pointer): Byte
    fun fatoora_InvoiceLineItem_has_tax_total(handle: Pointer): Byte
    fun fatoora_InvoiceLineItem_has_gross_amount(handle: Pointer): Byte
    fun fatoora_InvoiceLineItem_vat_category(handle: Pointer): FFIUint8
}

class InvoiceLineItem internal constructor (
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
            lib.fatoora_InvoiceLineItem_destroy(handle)
    }

    companion object {
        internal val libClass: Class<InvoiceLineItemLib> = InvoiceLineItemLib::class.java
        internal val lib: InvoiceLineItemLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(description: String, quantity: String, unitCode: String, unitPrice: String, rate: String, category: UByte): Result<InvoiceLineItem> {
            val descriptionSliceMemory = PrimitiveArrayTools.borrowUtf8(description)
            val quantitySliceMemory = PrimitiveArrayTools.borrowUtf8(quantity)
            val unitCodeSliceMemory = PrimitiveArrayTools.borrowUtf8(unitCode)
            val unitPriceSliceMemory = PrimitiveArrayTools.borrowUtf8(unitPrice)
            val rateSliceMemory = PrimitiveArrayTools.borrowUtf8(rate)

            val returnVal = lib.fatoora_InvoiceLineItem_new(descriptionSliceMemory.slice, quantitySliceMemory.slice, unitCodeSliceMemory.slice, unitPriceSliceMemory.slice, rateSliceMemory.slice, FFIUint8(category));
            try {
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
            } finally {
                descriptionSliceMemory.close()
                quantitySliceMemory.close()
                unitCodeSliceMemory.close()
                unitPriceSliceMemory.close()
                rateSliceMemory.close()
            }
        }
    }

    fun itemGrossPrice(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceLineItem_item_gross_price(handle);
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

    fun itemPriceDiscount(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceLineItem_item_price_discount(handle);
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

    fun itemPriceBaseQuantity(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceLineItem_item_price_base_quantity(handle);
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

    fun itemPriceBaseQuantityUnitCode(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceLineItem_item_price_base_quantity_unit_code(handle);
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

        val returnVal = lib.fatoora_InvoiceLineItem_adjustments_len(handle);
        return (returnVal.toULong())
    }

    fun adjustment(index: ULong): Result<Adjustment> {

        val returnVal = lib.fatoora_InvoiceLineItem_adjustment(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = Adjustment(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun adjustmentIsCharge(index: ULong): Result<Boolean> {

        val returnVal = lib.fatoora_InvoiceLineItem_adjustment_is_charge(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            return (nativeOkVal > 0).ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun itemSellerIdentifier(): Result<OtherId?> {

        val returnVal = lib.fatoora_InvoiceLineItem_item_seller_identifier(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = OtherId(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun itemBuyerIdentifier(): Result<OtherId?> {

        val returnVal = lib.fatoora_InvoiceLineItem_item_buyer_identifier(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = OtherId(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun itemStandardIdentifier(): Result<OtherId?> {

        val returnVal = lib.fatoora_InvoiceLineItem_item_standard_identifier(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = OtherId(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun withDocumentReference(reference: OriginalInvoiceRef): Result<InvoiceLineItem> {

        val returnVal = lib.fatoora_InvoiceLineItem_with_document_reference(handle, reference.handle);
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

    fun withTaxSubtotal(subtotal: VatBreakdown): Result<InvoiceLineItem> {

        val returnVal = lib.fatoora_InvoiceLineItem_with_tax_subtotal(handle, subtotal.handle);
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

    fun documentReferencesLen(): ULong {

        val returnVal = lib.fatoora_InvoiceLineItem_document_references_len(handle);
        return (returnVal.toULong())
    }

    fun documentReference(index: ULong): Result<OriginalInvoiceRef> {

        val returnVal = lib.fatoora_InvoiceLineItem_document_reference(handle, FFISizet(index));
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

    fun taxSubtotalsLen(): ULong {

        val returnVal = lib.fatoora_InvoiceLineItem_tax_subtotals_len(handle);
        return (returnVal.toULong())
    }

    fun taxSubtotal(index: ULong): Result<VatBreakdown> {

        val returnVal = lib.fatoora_InvoiceLineItem_tax_subtotal(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = VatBreakdown(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun withMetadata(id: String?, exemption: TaxExemption): Result<InvoiceLineItem> {
        val idSliceMemory = id?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_InvoiceLineItem_with_metadata(handle, idSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), exemption.handle);
        try {
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
        } finally {
            idSliceMemory?.close()
        }
    }

    fun id(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceLineItem_id(handle);
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

    fun exemption(): Result<TaxExemption> {

        val returnVal = lib.fatoora_InvoiceLineItem_exemption(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = TaxExemption(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun description(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceLineItem_description(handle, write);
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

    fun unitCode(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceLineItem_unit_code(handle, write);
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

    fun quantity(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceLineItem_quantity(handle, write);
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

    fun unitPrice(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceLineItem_unit_price(handle, write);
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

    fun totalAmount(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceLineItem_total_amount(handle, write);
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

    fun vatRate(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceLineItem_vat_rate(handle, write);
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

    fun vatAmount(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceLineItem_vat_amount(handle, write);
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

    fun hasVatRate(): Boolean {

        val returnVal = lib.fatoora_InvoiceLineItem_has_vat_rate(handle);
        return (returnVal > 0)
    }

    fun hasTaxTotal(): Boolean {

        val returnVal = lib.fatoora_InvoiceLineItem_has_tax_total(handle);
        return (returnVal > 0)
    }

    fun hasGrossAmount(): Boolean {

        val returnVal = lib.fatoora_InvoiceLineItem_has_gross_amount(handle);
        return (returnVal > 0)
    }

    fun vatCategory(): UByte {

        val returnVal = lib.fatoora_InvoiceLineItem_vat_category(handle);
        return (returnVal.toUByte())
    }

}
