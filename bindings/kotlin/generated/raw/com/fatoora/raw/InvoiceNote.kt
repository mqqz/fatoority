package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceNoteLib: Library {
    fun fatoora_InvoiceNote_destroy(handle: Pointer)
    fun fatoora_InvoiceNote_language_id(handle: Pointer): ResultPointerPointer
    fun fatoora_InvoiceNote_language(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_InvoiceNote_text(handle: Pointer, write: Pointer): ResultUnitPointer
}

class InvoiceNote internal constructor (
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
            lib.fatoora_InvoiceNote_destroy(handle)
    }

    companion object {
        internal val libClass: Class<InvoiceNoteLib> = InvoiceNoteLib::class.java
        internal val lib: InvoiceNoteLib = Native.load("fatoora_ffi", libClass)
    }

    fun languageId(): Result<Text?> {

        val returnVal = lib.fatoora_InvoiceNote_language_id(handle);
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

    fun language(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceNote_language(handle, write);
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

    fun text(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_InvoiceNote_text(handle, write);
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
