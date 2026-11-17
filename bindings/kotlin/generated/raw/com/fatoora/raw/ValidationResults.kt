package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ValidationResultsLib: Library {
    fun fatoora_ValidationResults_destroy(handle: Pointer)
    fun fatoora_ValidationResults_status(handle: Pointer): OptionInt
    fun fatoora_ValidationResults_status_raw(handle: Pointer): ResultPointerPointer
    fun fatoora_ValidationResults_info_len(handle: Pointer): FFISizet
    fun fatoora_ValidationResults_info_message(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_ValidationResults_warning_len(handle: Pointer): FFISizet
    fun fatoora_ValidationResults_warning_message(handle: Pointer, index: FFISizet): ResultPointerPointer
    fun fatoora_ValidationResults_error_len(handle: Pointer): FFISizet
    fun fatoora_ValidationResults_error_message(handle: Pointer, index: FFISizet): ResultPointerPointer
}

class ValidationResults internal constructor (
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
            lib.fatoora_ValidationResults_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ValidationResultsLib> = ValidationResultsLib::class.java
        internal val lib: ValidationResultsLib = Native.load("fatoora_ffi", libClass)
    }

    /**
     * The parsed overall status, or null when the response has none.
     */
    fun status(): ValidationStatus? {

        val returnVal = lib.fatoora_ValidationResults_status(handle);

        val intermediateOption = returnVal.option() ?: return null
        return ValidationStatus.fromNative(intermediateOption)
    }

    /**
     * The overall status exactly as ZATCA sent it, or null when absent.
     */
    fun statusRaw(): Result<Text?> {

        val returnVal = lib.fatoora_ValidationResults_status_raw(handle);
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

    fun infoLen(): ULong {

        val returnVal = lib.fatoora_ValidationResults_info_len(handle);
        return (returnVal.toULong())
    }

    fun infoMessage(index: ULong): Result<ValidationMessage> {

        val returnVal = lib.fatoora_ValidationResults_info_message(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ValidationMessage(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun warningLen(): ULong {

        val returnVal = lib.fatoora_ValidationResults_warning_len(handle);
        return (returnVal.toULong())
    }

    fun warningMessage(index: ULong): Result<ValidationMessage> {

        val returnVal = lib.fatoora_ValidationResults_warning_message(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ValidationMessage(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun errorLen(): ULong {

        val returnVal = lib.fatoora_ValidationResults_error_len(handle);
        return (returnVal.toULong())
    }

    fun errorMessage(index: ULong): Result<ValidationMessage> {

        val returnVal = lib.fatoora_ValidationResults_error_message(handle, FFISizet(index));
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = ValidationMessage(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
