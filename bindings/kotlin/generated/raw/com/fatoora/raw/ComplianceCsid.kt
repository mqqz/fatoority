package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ComplianceCsidLib: Library {
    fun fatoora_ComplianceCsid_destroy(handle: Pointer)
    fun fatoora_ComplianceCsid_new(environment: Int, requestId: OptionSlice, token: Slice, secret: Slice): ResultPointerPointer
    fun fatoora_ComplianceCsid_env(handle: Pointer): ResultIntPointer
    fun fatoora_ComplianceCsid_request_id(handle: Pointer, write: Pointer): OptionUnit
    fun fatoora_ComplianceCsid_binary_security_token(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ComplianceCsid_secret(handle: Pointer, write: Pointer): ResultUnitPointer
}

class ComplianceCsid internal constructor (
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
            lib.fatoora_ComplianceCsid_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ComplianceCsidLib> = ComplianceCsidLib::class.java
        internal val lib: ComplianceCsidLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(environment: Environment, requestId: String?, token: String, secret: String): Result<ComplianceCsid> {
            val requestIdSliceMemory = requestId?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val tokenSliceMemory = PrimitiveArrayTools.borrowUtf8(token)
            val secretSliceMemory = PrimitiveArrayTools.borrowUtf8(secret)

            val returnVal = lib.fatoora_ComplianceCsid_new(environment.toNative(), requestIdSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), tokenSliceMemory.slice, secretSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = ComplianceCsid(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                requestIdSliceMemory?.close()
                tokenSliceMemory.close()
                secretSliceMemory.close()
            }
        }
    }

    fun env(): Result<Environment> {

        val returnVal = lib.fatoora_ComplianceCsid_env(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            return (Environment.fromNative(nativeOkVal)).ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun requestId(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ComplianceCsid_request_id(handle, write);

        if (returnVal.option() == null) {
            DW.lib.diplomat_buffer_write_destroy(write)
            return null
        }

        val returnString = DW.writeToString(write)
        return returnString

    }

    fun binarySecurityToken(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ComplianceCsid_binary_security_token(handle, write);
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

    fun secret(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ComplianceCsid_secret(handle, write);
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
