package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZatcaValidationReportLib: Library {
    fun fatoora_ZatcaValidationReport_destroy(handle: Pointer)
    fun fatoora_ZatcaValidationReport_schema_version(handle: Pointer): FFIUint32
    fun fatoora_ZatcaValidationReport_profile(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ZatcaValidationReport_evaluated_at(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ZatcaValidationReport_is_complete(handle: Pointer): Byte
    fun fatoora_ZatcaValidationReport_has_errors(handle: Pointer): Byte
    fun fatoora_ZatcaValidationReport_is_valid(handle: Pointer): Byte
    fun fatoora_ZatcaValidationReport_stages_len(handle: Pointer): FFISizet
    fun fatoora_ZatcaValidationReport_stage(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_ZatcaValidationReport_to_json(handle: Pointer, write: Pointer): ResultUnitPointer
}
/**
 * Owned results of local ZATCA validation with explicit stage coverage.
 *
 * Completion and validity describe only the local profile; they do not
 * establish acceptance by ZATCA or certificate issuer trust.
 */
class ZatcaValidationReport internal constructor (
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
            lib.fatoora_ZatcaValidationReport_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ZatcaValidationReportLib> = ZatcaValidationReportLib::class.java
        internal val lib: ZatcaValidationReportLib = Native.load("fatoora_ffi", libClass)
    }

    /**
     * Version of the report's serialized format.
     */
    fun schemaVersion(): UInt {

        val returnVal = lib.fatoora_ZatcaValidationReport_schema_version(handle);
        return (returnVal.toUInt())
    }

    /**
     * Identifier of the validation profile (the ZATCA SDK release it mirrors).
     */
    fun profile(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaValidationReport_profile(handle, write);
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
     * RFC 3339 instant used for time-dependent rules.
     */
    fun evaluatedAt(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaValidationReport_evaluated_at(handle, write);
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
     * Whether every required stage finished exactly once.
     */
    fun isComplete(): Boolean {

        val returnVal = lib.fatoora_ZatcaValidationReport_is_complete(handle);
        return (returnVal > 0)
    }

    /**
     * Whether any stage, including an incomplete stage, reported an error.
     */
    fun hasErrors(): Boolean {

        val returnVal = lib.fatoora_ZatcaValidationReport_has_errors(handle);
        return (returnVal > 0)
    }

    /**
     * Whether validation is complete and contains no error findings.
     */
    fun isValid(): Boolean {

        val returnVal = lib.fatoora_ZatcaValidationReport_is_valid(handle);
        return (returnVal > 0)
    }

    fun stagesLen(): ULong {

        val returnVal = lib.fatoora_ZatcaValidationReport_stages_len(handle);
        return (returnVal.toULong())
    }

    /**
     * The stage report at `index`, in stage order.
     */
    fun stage(index: ULong): Result<ZatcaStageReport> {

        val returnVal = lib.fatoora_ZatcaValidationReport_stage(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ZatcaStageReport(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    /**
     * The report serialized as JSON, including the derived `is_complete`,
     * `has_errors` and `is_valid` fields.
     */
    fun toJson(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ZatcaValidationReport_to_json(handle, write);
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
