package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface AllowanceChargeLib: Library {
    fun fatoora_AllowanceCharge_destroy(handle: Pointer)
    fun fatoora_AllowanceCharge_has_vat_rate(handle: Pointer): Byte
    fun fatoora_AllowanceCharge_without_vat_rate(handle: Pointer): ResultPointerPointer
    fun fatoora_AllowanceCharge_with_percentage(handle: Pointer, base: Slice, percent: Slice): ResultPointerPointer
    fun fatoora_AllowanceCharge_base_amount(handle: Pointer): ResultPointerPointer
    fun fatoora_AllowanceCharge_percentage(handle: Pointer): ResultPointerPointer
    fun fatoora_AllowanceCharge_new(charge: Boolean, amount: Slice, reason: OptionSlice, reasonCode: OptionSlice, category: FFIUint8, rate: Slice, exemption: Pointer): ResultPointerPointer
    fun fatoora_AllowanceCharge_is_charge(handle: Pointer): Byte
    fun fatoora_AllowanceCharge_amount(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_AllowanceCharge_rate(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_AllowanceCharge_category(handle: Pointer): FFIUint8
    fun fatoora_AllowanceCharge_with_tax_category_count(handle: Pointer, count: FFISizet): ResultPointerPointer
    fun fatoora_AllowanceCharge_tax_category_count(handle: Pointer): FFISizet
    fun fatoora_AllowanceCharge_reason(handle: Pointer): ResultPointerPointer
    fun fatoora_AllowanceCharge_reason_code(handle: Pointer): ResultPointerPointer
    fun fatoora_AllowanceCharge_exemption(handle: Pointer): ResultPointerPointer
}

class AllowanceCharge internal constructor (
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
            lib.fatoora_AllowanceCharge_destroy(handle)
    }

    companion object {
        internal val libClass: Class<AllowanceChargeLib> = AllowanceChargeLib::class.java
        internal val lib: AllowanceChargeLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(charge: Boolean, amount: String, reason: String?, reasonCode: String?, category: UByte, rate: String, exemption: TaxExemption): Result<AllowanceCharge> {
            val amountSliceMemory = PrimitiveArrayTools.borrowUtf8(amount)
            val reasonSliceMemory = reason?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val reasonCodeSliceMemory = reasonCode?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val rateSliceMemory = PrimitiveArrayTools.borrowUtf8(rate)

            val returnVal = lib.fatoora_AllowanceCharge_new(charge, amountSliceMemory.slice, reasonSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), reasonCodeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), FFIUint8(category), rateSliceMemory.slice, exemption.handle);
            try {
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
            } finally {
                amountSliceMemory.close()
                reasonSliceMemory?.close()
                reasonCodeSliceMemory?.close()
                rateSliceMemory.close()
            }
        }
    }

    fun hasVatRate(): Boolean {

        val returnVal = lib.fatoora_AllowanceCharge_has_vat_rate(handle);
        return (returnVal > 0)
    }

    fun withoutVatRate(): Result<AllowanceCharge> {

        val returnVal = lib.fatoora_AllowanceCharge_without_vat_rate(handle);
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

    /**
     * Retain this adjustment's metadata and check its percentage calculation.
     */
    fun withPercentage(base: String, percent: String): Result<AllowanceCharge> {
        val baseSliceMemory = PrimitiveArrayTools.borrowUtf8(base)
        val percentSliceMemory = PrimitiveArrayTools.borrowUtf8(percent)

        val returnVal = lib.fatoora_AllowanceCharge_with_percentage(handle, baseSliceMemory.slice, percentSliceMemory.slice);
        try {
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
        } finally {
            baseSliceMemory.close()
            percentSliceMemory.close()
        }
    }

    fun baseAmount(): Result<Text?> {

        val returnVal = lib.fatoora_AllowanceCharge_base_amount(handle);
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

    fun percentage(): Result<Text?> {

        val returnVal = lib.fatoora_AllowanceCharge_percentage(handle);
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

    fun isCharge(): Boolean {

        val returnVal = lib.fatoora_AllowanceCharge_is_charge(handle);
        return (returnVal > 0)
    }

    fun amount(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_AllowanceCharge_amount(handle, write);
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

    fun rate(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_AllowanceCharge_rate(handle, write);
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

    fun category(): UByte {

        val returnVal = lib.fatoora_AllowanceCharge_category(handle);
        return (returnVal.toUByte())
    }

    /**
     * Return an independent adjustment with the given number of identical tax classifications.
     */
    fun withTaxCategoryCount(count: ULong): Result<AllowanceCharge> {

        val returnVal = lib.fatoora_AllowanceCharge_with_tax_category_count(handle, FFISizet(count));
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

    fun taxCategoryCount(): ULong {

        val returnVal = lib.fatoora_AllowanceCharge_tax_category_count(handle);
        return (returnVal.toULong())
    }

    fun reason(): Result<Text?> {

        val returnVal = lib.fatoora_AllowanceCharge_reason(handle);
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

    fun reasonCode(): Result<Text?> {

        val returnVal = lib.fatoora_AllowanceCharge_reason_code(handle);
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

        val returnVal = lib.fatoora_AllowanceCharge_exemption(handle);
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

}
