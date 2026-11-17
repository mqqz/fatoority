package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface FinalizedInvoiceLib: Library {
    fun fatoora_FinalizedInvoice_destroy(handle: Pointer)
    fun fatoora_FinalizedInvoice_from_xml(value: Slice): ResultPointerPointer
    fun fatoora_FinalizedInvoice_from_file(value: Slice): ResultPointerPointer
    fun fatoora_FinalizedInvoice_data(handle: Pointer): ResultPointerPointer
    fun fatoora_FinalizedInvoice_totals(handle: Pointer): ResultPointerPointer
    fun fatoora_FinalizedInvoice_hash(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_FinalizedInvoice_xml(handle: Pointer, write: Pointer): ResultUnitPointer
}

class FinalizedInvoice internal constructor (
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
            lib.fatoora_FinalizedInvoice_destroy(handle)
    }

    companion object {
        internal val libClass: Class<FinalizedInvoiceLib> = FinalizedInvoiceLib::class.java
        internal val lib: FinalizedInvoiceLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun fromXml(value: String): Result<FinalizedInvoice> {
            val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

            val returnVal = lib.fatoora_FinalizedInvoice_from_xml(valueSliceMemory.slice);
            try {
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
            } finally {
                valueSliceMemory.close()
            }
        }
        @JvmStatic

        fun fromFile(value: String): Result<FinalizedInvoice> {
            val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

            val returnVal = lib.fatoora_FinalizedInvoice_from_file(valueSliceMemory.slice);
            try {
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
            } finally {
                valueSliceMemory.close()
            }
        }
    }

    fun data(): Result<InvoiceData> {

        val returnVal = lib.fatoora_FinalizedInvoice_data(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = InvoiceData(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun totals(): Result<InvoiceTotals> {

        val returnVal = lib.fatoora_FinalizedInvoice_totals(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = InvoiceTotals(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun hash(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_FinalizedInvoice_hash(handle, write);
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

    fun xml(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_FinalizedInvoice_xml(handle, write);
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

}
