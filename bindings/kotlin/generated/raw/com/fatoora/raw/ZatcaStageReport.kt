package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZatcaStageReportLib: Library {
    fun fatoora_ZatcaStageReport_destroy(handle: Pointer)
    fun fatoora_ZatcaStageReport_stage(handle: Pointer): Int
    fun fatoora_ZatcaStageReport_status(handle: Pointer): Int
    fun fatoora_ZatcaStageReport_provenance(handle: Pointer): ResultPointerPointer
    fun fatoora_ZatcaStageReport_evaluated_assertions_len(handle: Pointer): FFISizet
    fun fatoora_ZatcaStageReport_evaluated_assertion(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_ZatcaStageReport_findings_len(handle: Pointer): FFISizet
    fun fatoora_ZatcaStageReport_finding(handle: Pointer, index: FFISizet): ResultPointerPointer
}
/**
 * Coverage and findings from one validation stage.
 */
class ZatcaStageReport internal constructor (
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
            lib.fatoora_ZatcaStageReport_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ZatcaStageReportLib> = ZatcaStageReportLib::class.java
        internal val lib: ZatcaStageReportLib = Native.load("fatoora_ffi", libClass)
    }

    fun stage(): ZatcaStage {

        val returnVal = lib.fatoora_ZatcaStageReport_stage(handle);
        return (ZatcaStage.fromNative(returnVal))
    }

    fun status(): ZatcaStageStatus {

        val returnVal = lib.fatoora_ZatcaStageReport_status(handle);
        return (ZatcaStageStatus.fromNative(returnVal))
    }

    /**
     * Pinned rule source, for the CEN and KSA business-rule stages.
     */
    fun provenance(): Result<ZatcaRuleSource?> {

        val returnVal = lib.fatoora_ZatcaStageReport_provenance(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = ZatcaRuleSource(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun evaluatedAssertionsLen(): ULong {

        val returnVal = lib.fatoora_ZatcaStageReport_evaluated_assertions_len(handle);
        return (returnVal.toULong())
    }

    /**
     * Assertion site at `index` whose selected contexts were all evaluated.
     */
    fun evaluatedAssertion(index: ULong): Result<Text> {

        val returnVal = lib.fatoora_ZatcaStageReport_evaluated_assertion(handle, FFISizet(index));
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

    fun findingsLen(): ULong {

        val returnVal = lib.fatoora_ZatcaStageReport_findings_len(handle);
        return (returnVal.toULong())
    }

    fun finding(index: ULong): Result<ZatcaFinding> {

        val returnVal = lib.fatoora_ZatcaStageReport_finding(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ZatcaFinding(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
