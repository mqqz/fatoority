package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZatcaFindingLib: Library {
    fun fatoora_ZatcaFinding_destroy(handle: Pointer)
    fun fatoora_ZatcaFinding_assertion_site(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ZatcaFinding_layer(handle: Pointer): Int
    fun fatoora_ZatcaFinding_code(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ZatcaFinding_severity(handle: Pointer): Int
    fun fatoora_ZatcaFinding_message(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ZatcaFinding_location_kind(handle: Pointer): OptionInt
    fun fatoora_ZatcaFinding_location_path(handle: Pointer): ResultPointerPointer
    fun fatoora_ZatcaFinding_location_line(handle: Pointer): OptionFFIUint32
    fun fatoora_ZatcaFinding_location_column(handle: Pointer): OptionFFIUint32
}
/**
 * One validation finding with its source assertion, when applicable.
 */
class ZatcaFinding internal constructor (
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
            lib.fatoora_ZatcaFinding_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ZatcaFindingLib> = ZatcaFindingLib::class.java
        internal val lib: ZatcaFindingLib = Native.load("fatoora_ffi", libClass)
    }

    /**
     * Rule assertion that produced the finding, for business-rule findings.
     */
    fun assertionSite(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaFinding_assertion_site(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    fun layer(): ValidationLayer {

        val returnVal = lib.fatoora_ZatcaFinding_layer(handle);
        return (ValidationLayer.fromNative(returnVal))
    }

    /**
     * Library-owned rule identifier, independent of message wording.
     */
    fun code(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaFinding_code(handle, write);
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

    fun severity(): Severity {

        val returnVal = lib.fatoora_ZatcaFinding_severity(handle);
        return (Severity.fromNative(returnVal))
    }

    /**
     * Human-readable text; the wording is not a stable API.
     */
    fun message(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaFinding_message(handle, write);
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
     * The kind of location, or null when the source cannot locate the finding.
     */
    fun locationKind(): ValidationLocationKind? {

        val returnVal = lib.fatoora_ZatcaFinding_location_kind(handle);

        val intermediateOption = returnVal.option() ?: return null
        return ValidationLocationKind.fromNative(intermediateOption)
    }

    /**
     * The field path or XPath, or null for other location kinds.
     */
    fun locationPath(): Result<Text?> {

        val returnVal = lib.fatoora_ZatcaFinding_location_path(handle);
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

    /**
     * One-based XML line, or null when the location is not XML coordinates.
     */
    fun locationLine(): UInt? {

        val returnVal = lib.fatoora_ZatcaFinding_location_line(handle);
        return returnVal.option()?.toUInt()
    }

    /**
     * One-based XML column, or null when unknown.
     */
    fun locationColumn(): UInt? {

        val returnVal = lib.fatoora_ZatcaFinding_location_column(handle);
        return returnVal.option()?.toUInt()
    }

}
