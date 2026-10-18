package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TaxExemptionLib: Library {
    fun fatoora_TaxExemption_destroy(handle: Pointer)
    fun fatoora_TaxExemption_new(code: OptionSlice, reason: OptionSlice): ResultPointerPointer
    fun fatoora_TaxExemption_reason_code(handle: Pointer): ResultPointerPointer
    fun fatoora_TaxExemption_reason(handle: Pointer): ResultPointerPointer
}

class TaxExemption internal constructor (
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
            lib.fatoora_TaxExemption_destroy(handle)
    }

    companion object {
        internal val libClass: Class<TaxExemptionLib> = TaxExemptionLib::class.java
        internal val lib: TaxExemptionLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(code: String?, reason: String?): Result<TaxExemption> {
            val codeSliceMemory = code?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val reasonSliceMemory = reason?.let { PrimitiveArrayTools.borrowUtf8(it) }

            val returnVal = lib.fatoora_TaxExemption_new(codeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), reasonSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
            try {
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
            } finally {
                codeSliceMemory?.close()
                reasonSliceMemory?.close()
            }
        }
    }

    fun reasonCode(): Result<Text?> {

        val returnVal = lib.fatoora_TaxExemption_reason_code(handle);
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

    fun reason(): Result<Text?> {

        val returnVal = lib.fatoora_TaxExemption_reason(handle);
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
