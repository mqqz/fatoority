package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PaymentMeansLib: Library {
    fun fatoora_PaymentMeans_destroy(handle: Pointer)
    fun fatoora_PaymentMeans_with_instruction_note(handle: Pointer, value: Slice): ResultPointerPointer
    fun fatoora_PaymentMeans_instruction_notes_len(handle: Pointer): FFISizet
    fun fatoora_PaymentMeans_instruction_note_at(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_PaymentMeans_new(code: Slice, instruction: OptionSlice): ResultPointerPointer
    fun fatoora_PaymentMeans_with_account(handle: Pointer, id: OptionSlice, terms: OptionSlice): ResultPointerPointer
    fun fatoora_PaymentMeans_code(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_PaymentMeans_instruction_note(handle: Pointer): ResultPointerPointer
    fun fatoora_PaymentMeans_has_account(handle: Pointer): Byte
    fun fatoora_PaymentMeans_account_identifier(handle: Pointer): ResultPointerPointer
    fun fatoora_PaymentMeans_payment_terms(handle: Pointer): ResultPointerPointer
}

class PaymentMeans internal constructor (
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
            lib.fatoora_PaymentMeans_destroy(handle)
    }

    companion object {
        internal val libClass: Class<PaymentMeansLib> = PaymentMeansLib::class.java
        internal val lib: PaymentMeansLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(code: String, instruction: String?): Result<PaymentMeans> {
            val codeSliceMemory = PrimitiveArrayTools.borrowUtf8(code)
            val instructionSliceMemory = instruction?.let { PrimitiveArrayTools.borrowUtf8(it) }

            val returnVal = lib.fatoora_PaymentMeans_new(codeSliceMemory.slice, instructionSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
            try {
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
            } finally {
                codeSliceMemory.close()
                instructionSliceMemory?.close()
            }
        }
    }

    fun withInstructionNote(value: String): Result<PaymentMeans> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_PaymentMeans_with_instruction_note(handle, valueSliceMemory.slice);
        try {
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
        } finally {
            valueSliceMemory.close()
        }
    }

    fun instructionNotesLen(): ULong {

        val returnVal = lib.fatoora_PaymentMeans_instruction_notes_len(handle);
        return (returnVal.toULong())
    }

    fun instructionNoteAt(index: ULong): Result<Text> {

        val returnVal = lib.fatoora_PaymentMeans_instruction_note_at(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun withAccount(id: String?, terms: String?): Result<PaymentMeans> {
        val idSliceMemory = id?.let { PrimitiveArrayTools.borrowUtf8(it) }
        val termsSliceMemory = terms?.let { PrimitiveArrayTools.borrowUtf8(it) }

        val returnVal = lib.fatoora_PaymentMeans_with_account(handle, idSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), termsSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
        try {
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
        } finally {
            idSliceMemory?.close()
            termsSliceMemory?.close()
        }
    }

    fun code(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_PaymentMeans_code(handle, write);
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

    fun instructionNote(): Result<Text?> {

        val returnVal = lib.fatoora_PaymentMeans_instruction_note(handle);
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

    fun hasAccount(): Boolean {

        val returnVal = lib.fatoora_PaymentMeans_has_account(handle);
        return (returnVal > 0)
    }

    fun accountIdentifier(): Result<Text?> {

        val returnVal = lib.fatoora_PaymentMeans_account_identifier(handle);
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

    fun paymentTerms(): Result<Text?> {

        val returnVal = lib.fatoora_PaymentMeans_payment_terms(handle);
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
