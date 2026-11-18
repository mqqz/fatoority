package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OtherIdLib: Library {
    fun fatoora_OtherId_destroy(handle: Pointer)
    fun fatoora_OtherId_value(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_OtherId_scheme(handle: Pointer, write: Pointer): OptionUnit
}

class OtherId internal constructor (
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
            lib.fatoora_OtherId_destroy(handle)
    }

    companion object {
        internal val libClass: Class<OtherIdLib> = OtherIdLib::class.java
        internal val lib: OtherIdLib = Native.load("fatoora_ffi", libClass)
    }

    fun value(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_OtherId_value(handle, write);
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

    fun scheme(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_OtherId_scheme(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

}
