package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZatcaRuleSourceLib: Library {
    fun fatoora_ZatcaRuleSource_destroy(handle: Pointer)
    fun fatoora_ZatcaRuleSource_stylesheet(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ZatcaRuleSource_sha256(handle: Pointer, write: Pointer): ResultUnitPointer
}
/**
 * Pinned upstream rule source of a business-rule stage.
 */
class ZatcaRuleSource internal constructor (
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
            lib.fatoora_ZatcaRuleSource_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ZatcaRuleSourceLib> = ZatcaRuleSourceLib::class.java
        internal val lib: ZatcaRuleSourceLib = Native.load("fatoora_ffi", libClass)
    }

    /**
     * File name of the upstream rule stylesheet.
     */
    fun stylesheet(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaRuleSource_stylesheet(handle, write);
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

    /**
     * Hex SHA-256 digest of the pinned stylesheet.
     */
    fun sha256(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaRuleSource_sha256(handle, write);
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
