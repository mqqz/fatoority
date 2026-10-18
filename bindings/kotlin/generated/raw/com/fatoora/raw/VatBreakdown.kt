package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface VatBreakdownLib: Library {
    fun fatoora_VatBreakdown_destroy(handle: Pointer)
    fun fatoora_VatBreakdown_new(category: FFIUint8, rate: Slice, taxableAmount: Slice, taxAmount: Slice, exemption: Pointer): ResultPointerPointer
    fun fatoora_VatBreakdown_category(handle: Pointer): FFIUint8
    fun fatoora_VatBreakdown_rate(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_VatBreakdown_taxable_amount(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_VatBreakdown_tax_amount(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_VatBreakdown_exemption(handle: Pointer): ResultPointerPointer
}

class VatBreakdown internal constructor (
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
            lib.fatoora_VatBreakdown_destroy(handle)
    }

    companion object {
        internal val libClass: Class<VatBreakdownLib> = VatBreakdownLib::class.java
        internal val lib: VatBreakdownLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(category: UByte, rate: String, taxableAmount: String, taxAmount: String, exemption: TaxExemption): Result<VatBreakdown> {
            val rateSliceMemory = PrimitiveArrayTools.borrowUtf8(rate)
            val taxableAmountSliceMemory = PrimitiveArrayTools.borrowUtf8(taxableAmount)
            val taxAmountSliceMemory = PrimitiveArrayTools.borrowUtf8(taxAmount)

            val returnVal = lib.fatoora_VatBreakdown_new(FFIUint8(category), rateSliceMemory.slice, taxableAmountSliceMemory.slice, taxAmountSliceMemory.slice, exemption.handle);
            try {
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
            } finally {
                rateSliceMemory.close()
                taxableAmountSliceMemory.close()
                taxAmountSliceMemory.close()
            }
        }
    }

    fun category(): UByte {

        val returnVal = lib.fatoora_VatBreakdown_category(handle);
        return (returnVal.toUByte())
    }

    fun rate(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_VatBreakdown_rate(handle, write);
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

    fun taxableAmount(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_VatBreakdown_taxable_amount(handle, write);
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

    fun taxAmount(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_VatBreakdown_tax_amount(handle, write);
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

    fun exemption(): Result<TaxExemption> {

        val returnVal = lib.fatoora_VatBreakdown_exemption(handle);
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
