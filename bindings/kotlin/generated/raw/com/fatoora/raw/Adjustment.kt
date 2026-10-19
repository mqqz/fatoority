package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface AdjustmentLib: Library {
    fun fatoora_Adjustment_destroy(handle: Pointer)
    fun fatoora_Adjustment_amount(value: Slice): ResultPointerPointer
    fun fatoora_Adjustment_percentage(base: Slice, percent: Slice): ResultPointerPointer
    fun fatoora_Adjustment_with_supplied_amount(handle: Pointer, value: Slice): ResultPointerPointer
    fun fatoora_Adjustment_with_reason(handle: Pointer, reason: OptionSlice, code: OptionSlice): ResultPointerPointer
    fun fatoora_Adjustment_value(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Adjustment_base_amount(handle: Pointer): ResultPointerPointer
    fun fatoora_Adjustment_percent(handle: Pointer): ResultPointerPointer
    fun fatoora_Adjustment_reason(handle: Pointer): ResultPointerPointer
    fun fatoora_Adjustment_reason_code(handle: Pointer): ResultPointerPointer
}

class Adjustment internal constructor (
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
            lib.fatoora_Adjustment_destroy(handle)
    }

    companion object {
        internal val libClass: Class<AdjustmentLib> = AdjustmentLib::class.java
        internal val lib: AdjustmentLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun amount(value: String): Result<Adjustment> {
            val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

            val returnVal = lib.fatoora_Adjustment_amount(valueSliceMemory.slice);
            try {
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
            } finally {
                valueSliceMemory.close()
            }
        }
        @JvmStatic

        fun percentage(base: String, percent: String): Result<Adjustment> {
            val baseSliceMemory = PrimitiveArrayTools.borrowUtf8(base)
            val percentSliceMemory = PrimitiveArrayTools.borrowUtf8(percent)

            val returnVal = lib.fatoora_Adjustment_percentage(baseSliceMemory.slice, percentSliceMemory.slice);
            try {
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
            } finally {
                baseSliceMemory.close()
                percentSliceMemory.close()
            }
        }
    }

    fun withSuppliedAmount(value: String): Result<Adjustment> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_Adjustment_with_supplied_amount(handle, valueSliceMemory.slice);
        try {
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
        } finally {
            valueSliceMemory.close()
        }
    }

    fun withReason(reason: String?, code: String?): Result<Adjustment> {
        val reasonSliceMemory = reason?.let { PrimitiveArrayTools.borrowUtf8(it) }
        val codeSliceMemory = code?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_Adjustment_with_reason(handle, reasonSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), codeSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
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
        } finally {
            reasonSliceMemory?.close()
            codeSliceMemory?.close()
        }
    }

    fun value(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Adjustment_value(handle, write);
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

    fun baseAmount(): Result<Text?> {

        val returnVal = lib.fatoora_Adjustment_base_amount(handle);
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

    fun percent(): Result<Text?> {

        val returnVal = lib.fatoora_Adjustment_percent(handle);
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

        val returnVal = lib.fatoora_Adjustment_reason(handle);
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

        val returnVal = lib.fatoora_Adjustment_reason_code(handle);
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
