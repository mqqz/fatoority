package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LineItemBuilderLib: Library {
    fun fatoora_LineItemBuilder_destroy(handle: Pointer)
    fun fatoora_LineItemBuilder_set_tax_total_present(handle: Pointer, present: Boolean): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_gross_amount_present(handle: Pointer, present: Boolean): ResultUnitPointer
    fun fatoora_LineItemBuilder_new(): Pointer
    fun fatoora_LineItemBuilder_build(handle: Pointer): ResultPointerPointer
    fun fatoora_LineItemBuilder_set_item_name(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_invoiced_quantity_unit_code(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_id(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_invoiced_quantity(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_net_price(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_gross_price(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_price_discount(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_price_base_quantity(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_line_net_amount(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_line_vat_amount(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_line_amount_with_vat(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_vat_rate(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_vat_category(handle: Pointer, value: FFIUint8): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_exemption(handle: Pointer, value: Pointer): ResultUnitPointer
    fun fatoora_LineItemBuilder_add_allowance(handle: Pointer, value: Pointer): ResultUnitPointer
    fun fatoora_LineItemBuilder_add_charge(handle: Pointer, value: Pointer): ResultUnitPointer
    fun fatoora_LineItemBuilder_add_document_reference(handle: Pointer, value: Pointer): ResultUnitPointer
    fun fatoora_LineItemBuilder_add_tax_subtotal(handle: Pointer, value: Pointer): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_seller_identifier(handle: Pointer, value: Slice, scheme: OptionSlice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_buyer_identifier(handle: Pointer, value: Slice, scheme: OptionSlice): ResultUnitPointer
    fun fatoora_LineItemBuilder_set_item_standard_identifier(handle: Pointer, value: Slice, scheme: OptionSlice): ResultUnitPointer
}

class LineItemBuilder internal constructor (
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
            lib.fatoora_LineItemBuilder_destroy(handle)
    }

    companion object {
        internal val libClass: Class<LineItemBuilderLib> = LineItemBuilderLib::class.java
        internal val lib: LineItemBuilderLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(): LineItemBuilder {

            val returnVal = lib.fatoora_LineItemBuilder_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal
            val returnOpaque = LineItemBuilder(handle, selfEdges, true)
            return returnOpaque
        }
    }

    fun setTaxTotalPresent(present: Boolean): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_set_tax_total_present(handle, present);
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

    fun setGrossAmountPresent(present: Boolean): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_set_gross_amount_present(handle, present);
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

    fun build(): Result<InvoiceLineItem> {

        val returnVal = lib.fatoora_LineItemBuilder_build(handle);
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

    fun setItemName(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_item_name(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setInvoicedQuantityUnitCode(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_invoiced_quantity_unit_code(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setItemPriceBaseQuantityUnitCode(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setId(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_id(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setInvoicedQuantity(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_invoiced_quantity(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setItemNetPrice(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_item_net_price(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setItemGrossPrice(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_item_gross_price(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setItemPriceDiscount(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_item_price_discount(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setItemPriceBaseQuantity(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_item_price_base_quantity(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setLineNetAmount(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_line_net_amount(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setLineVatAmount(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_line_vat_amount(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setLineAmountWithVat(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_line_amount_with_vat(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setVatRate(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_LineItemBuilder_set_vat_rate(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun setVatCategory(value: UByte): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_set_vat_category(handle, FFIUint8(value));
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

    fun setExemption(value: TaxExemption): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_set_exemption(handle, value.handle);
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

    fun addAllowance(value: Adjustment): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_add_allowance(handle, value.handle);
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

    fun addCharge(value: Adjustment): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_add_charge(handle, value.handle);
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

    fun addDocumentReference(value: OriginalInvoiceRef): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_add_document_reference(handle, value.handle);
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

    fun addTaxSubtotal(value: VatBreakdown): Result<Unit> {

        val returnVal = lib.fatoora_LineItemBuilder_add_tax_subtotal(handle, value.handle);
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

    fun setItemSellerIdentifier(value: String, scheme: String?): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)
        val schemeSliceMemory = scheme?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_LineItemBuilder_set_item_seller_identifier(handle, valueSliceMemory.slice, schemeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
            schemeSliceMemory?.close()
        }
    }

    fun setItemBuyerIdentifier(value: String, scheme: String?): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)
        val schemeSliceMemory = scheme?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_LineItemBuilder_set_item_buyer_identifier(handle, valueSliceMemory.slice, schemeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
            schemeSliceMemory?.close()
        }
    }

    fun setItemStandardIdentifier(value: String, scheme: String?): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)
        val schemeSliceMemory = scheme?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_LineItemBuilder_set_item_standard_identifier(handle, valueSliceMemory.slice, schemeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
            schemeSliceMemory?.close()
        }
    }

}
