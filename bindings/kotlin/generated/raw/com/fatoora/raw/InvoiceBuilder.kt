package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceBuilderLib: Library {
    fun fatoora_InvoiceBuilder_destroy(handle: Pointer)
    fun fatoora_InvoiceBuilder_set_payment_instruction(handle: Pointer, instruction: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_prepaid_amount(handle: Pointer, amount: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_payable_rounding_amount(handle: Pointer, amount: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_delivery(handle: Pointer, actual: OptionSlice, latest: OptionSlice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_tax_currency(handle: Pointer, currency: Slice, amount: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_tax_exemption(handle: Pointer, category: FFIUint8, rate: Slice, exemption: Pointer): ResultUnitPointer
    fun fatoora_InvoiceBuilder_add_adjustment(handle: Pointer, adjustment: Pointer): ResultUnitPointer
    fun fatoora_InvoiceBuilder_add_item(handle: Pointer, item: Pointer): ResultUnitPointer
    fun fatoora_InvoiceBuilder_new(kind: FFIUint8, subtype: FFIUint8, originalId: OptionSlice, originalUuid: OptionSlice, originalDate: OptionSlice, reason: OptionSlice): ResultPointerPointer
    fun fatoora_InvoiceBuilder_set_id(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_uuid(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_issue_datetime(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_currency(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_previous_invoice_hash(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_payment_means_code(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_allowance_reason(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_invoice_level_charge(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_invoice_level_discount(handle: Pointer, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_invoice_counter(handle: Pointer, value: FFIUint64): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_vat_category(handle: Pointer, value: FFIUint8): ResultUnitPointer
    fun fatoora_InvoiceBuilder_flags(handle: Pointer, value: FFIUint8): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_note(handle: Pointer, language: Slice, value: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_allowance(handle: Pointer, reason: Slice, amount: Slice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_seller(handle: Pointer, name: Slice, address: Pointer, vatId: Slice, otherId: OptionSlice, scheme: OptionSlice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_set_buyer(handle: Pointer, name: Slice, address: Pointer, vatId: OptionSlice, otherId: OptionSlice, scheme: OptionSlice): ResultUnitPointer
    fun fatoora_InvoiceBuilder_add_line_item(handle: Pointer, description: Slice, quantity: Slice, unitCode: Slice, unitPrice: Slice, vatRate: Slice, category: FFIUint8): ResultUnitPointer
    fun fatoora_InvoiceBuilder_build(handle: Pointer): ResultPointerPointer
}

class InvoiceBuilder internal constructor (
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
            lib.fatoora_InvoiceBuilder_destroy(handle)
    }

    companion object {
        internal val libClass: Class<InvoiceBuilderLib> = InvoiceBuilderLib::class.java
        internal val lib: InvoiceBuilderLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(kind: UByte, subtype: UByte, originalId: String?, originalUuid: String?, originalDate: String?, reason: String?): Result<InvoiceBuilder> {
            val originalIdSliceMemory = originalId?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val originalUuidSliceMemory = originalUuid?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val originalDateSliceMemory = originalDate?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val reasonSliceMemory = reason?.let { PrimitiveArrayTools.borrowUtf8(it) }

            val returnVal = lib.fatoora_InvoiceBuilder_new(FFIUint8(kind), FFIUint8(subtype), originalIdSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), originalUuidSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), originalDateSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), reasonSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = InvoiceBuilder(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                originalIdSliceMemory?.close()
                originalUuidSliceMemory?.close()
                originalDateSliceMemory?.close()
                reasonSliceMemory?.close()
            }
        }
    }

    fun setPaymentInstruction(instruction: String): Result<Unit> {
        val instructionSliceMemory = PrimitiveArrayTools.borrowUtf8(instruction)

        val returnVal = lib.fatoora_InvoiceBuilder_set_payment_instruction(handle, instructionSliceMemory.slice);
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
            instructionSliceMemory.close()
        }
    }

    fun setPrepaidAmount(amount: String): Result<Unit> {
        val amountSliceMemory = PrimitiveArrayTools.borrowUtf8(amount)

        val returnVal = lib.fatoora_InvoiceBuilder_set_prepaid_amount(handle, amountSliceMemory.slice);
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
            amountSliceMemory.close()
        }
    }

    fun setPayableRoundingAmount(amount: String): Result<Unit> {
        val amountSliceMemory = PrimitiveArrayTools.borrowUtf8(amount)

        val returnVal = lib.fatoora_InvoiceBuilder_set_payable_rounding_amount(handle, amountSliceMemory.slice);
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
            amountSliceMemory.close()
        }
    }

    fun setDelivery(actual: String?, latest: String?): Result<Unit> {
        val actualSliceMemory = actual?.let { PrimitiveArrayTools.borrowUtf8(it) }
        val latestSliceMemory = latest?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_InvoiceBuilder_set_delivery(handle, actualSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), latestSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
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
            actualSliceMemory?.close()
            latestSliceMemory?.close()
        }
    }

    fun setTaxCurrency(currency: String, amount: String): Result<Unit> {
        val currencySliceMemory = PrimitiveArrayTools.borrowUtf8(currency)
        val amountSliceMemory = PrimitiveArrayTools.borrowUtf8(amount)

        val returnVal = lib.fatoora_InvoiceBuilder_set_tax_currency(handle, currencySliceMemory.slice, amountSliceMemory.slice);
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
            currencySliceMemory.close()
            amountSliceMemory.close()
        }
    }

    fun setTaxExemption(category: UByte, rate: String, exemption: TaxExemption): Result<Unit> {
        val rateSliceMemory = PrimitiveArrayTools.borrowUtf8(rate)

        val returnVal = lib.fatoora_InvoiceBuilder_set_tax_exemption(handle, FFIUint8(category), rateSliceMemory.slice, exemption.handle);
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
            rateSliceMemory.close()
        }
    }

    fun addAdjustment(adjustment: AllowanceCharge): Result<Unit> {

        val returnVal = lib.fatoora_InvoiceBuilder_add_adjustment(handle, adjustment.handle);
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

    fun addItem(item: InvoiceLineItem): Result<Unit> {

        val returnVal = lib.fatoora_InvoiceBuilder_add_item(handle, item.handle);
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

    fun setId(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_set_id(handle, valueSliceMemory.slice);
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

    fun setUuid(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_set_uuid(handle, valueSliceMemory.slice);
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

    fun setIssueDatetime(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_set_issue_datetime(handle, valueSliceMemory.slice);
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

    fun setCurrency(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_set_currency(handle, valueSliceMemory.slice);
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

    fun setPreviousInvoiceHash(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_set_previous_invoice_hash(handle, valueSliceMemory.slice);
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

    fun setPaymentMeansCode(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_set_payment_means_code(handle, valueSliceMemory.slice);
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

    fun allowanceReason(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_allowance_reason(handle, valueSliceMemory.slice);
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

    fun invoiceLevelCharge(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_invoice_level_charge(handle, valueSliceMemory.slice);
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

    fun invoiceLevelDiscount(value: String): Result<Unit> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_invoice_level_discount(handle, valueSliceMemory.slice);
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

    fun setInvoiceCounter(value: ULong): Result<Unit> {

        val returnVal = lib.fatoora_InvoiceBuilder_set_invoice_counter(handle, FFIUint64(value));
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

    fun setVatCategory(value: UByte): Result<Unit> {

        val returnVal = lib.fatoora_InvoiceBuilder_set_vat_category(handle, FFIUint8(value));
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

    fun flags(value: UByte): Result<Unit> {

        val returnVal = lib.fatoora_InvoiceBuilder_flags(handle, FFIUint8(value));
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

    fun setNote(language: String, value: String): Result<Unit> {
        val languageSliceMemory = PrimitiveArrayTools.borrowUtf8(language)
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_InvoiceBuilder_set_note(handle, languageSliceMemory.slice, valueSliceMemory.slice);
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
            languageSliceMemory.close()
            valueSliceMemory.close()
        }
    }

    fun setAllowance(reason: String, amount: String): Result<Unit> {
        val reasonSliceMemory = PrimitiveArrayTools.borrowUtf8(reason)
        val amountSliceMemory = PrimitiveArrayTools.borrowUtf8(amount)

        val returnVal = lib.fatoora_InvoiceBuilder_set_allowance(handle, reasonSliceMemory.slice, amountSliceMemory.slice);
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
            reasonSliceMemory.close()
            amountSliceMemory.close()
        }
    }

    fun setSeller(name: String, address: Address, vatId: String, otherId: String?, scheme: String?): Result<Unit> {
        val nameSliceMemory = PrimitiveArrayTools.borrowUtf8(name)
        val vatIdSliceMemory = PrimitiveArrayTools.borrowUtf8(vatId)
        val otherIdSliceMemory = otherId?.let { PrimitiveArrayTools.borrowUtf8(it) }
        val schemeSliceMemory = scheme?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_InvoiceBuilder_set_seller(handle, nameSliceMemory.slice, address.handle, vatIdSliceMemory.slice, otherIdSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), schemeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
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
            nameSliceMemory.close()
            vatIdSliceMemory.close()
            otherIdSliceMemory?.close()
            schemeSliceMemory?.close()
        }
    }

    fun setBuyer(name: String, address: Address, vatId: String?, otherId: String?, scheme: String?): Result<Unit> {
        val nameSliceMemory = PrimitiveArrayTools.borrowUtf8(name)
        val vatIdSliceMemory = vatId?.let { PrimitiveArrayTools.borrowUtf8(it) }
        val otherIdSliceMemory = otherId?.let { PrimitiveArrayTools.borrowUtf8(it) }
        val schemeSliceMemory = scheme?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_InvoiceBuilder_set_buyer(handle, nameSliceMemory.slice, address.handle, vatIdSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), otherIdSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), schemeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
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
            nameSliceMemory.close()
            vatIdSliceMemory?.close()
            otherIdSliceMemory?.close()
            schemeSliceMemory?.close()
        }
    }

    fun addLineItem(description: String, quantity: String, unitCode: String, unitPrice: String, vatRate: String, category: UByte): Result<Unit> {
        val descriptionSliceMemory = PrimitiveArrayTools.borrowUtf8(description)
        val quantitySliceMemory = PrimitiveArrayTools.borrowUtf8(quantity)
        val unitCodeSliceMemory = PrimitiveArrayTools.borrowUtf8(unitCode)
        val unitPriceSliceMemory = PrimitiveArrayTools.borrowUtf8(unitPrice)
        val vatRateSliceMemory = PrimitiveArrayTools.borrowUtf8(vatRate)

        val returnVal = lib.fatoora_InvoiceBuilder_add_line_item(handle, descriptionSliceMemory.slice, quantitySliceMemory.slice, unitCodeSliceMemory.slice, unitPriceSliceMemory.slice, vatRateSliceMemory.slice, FFIUint8(category));
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
            descriptionSliceMemory.close()
            quantitySliceMemory.close()
            unitCodeSliceMemory.close()
            unitPriceSliceMemory.close()
            vatRateSliceMemory.close()
        }
    }

    fun build(): Result<FinalizedInvoice> {

        val returnVal = lib.fatoora_InvoiceBuilder_build(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = FinalizedInvoice(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
