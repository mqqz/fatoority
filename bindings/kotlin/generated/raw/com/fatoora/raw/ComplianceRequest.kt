package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ComplianceRequestLib: Library {
    fun fatoora_ComplianceRequest_destroy(handle: Pointer)
    fun fatoora_ComplianceRequest_method(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ComplianceRequest_url(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ComplianceRequest_headers_json(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_ComplianceRequest_body(handle: Pointer): ResultPointerPointer
    fun fatoora_ComplianceRequest_decode(handle: Pointer, status: FFIUint16, body: Slice): ResultPointerPointer
}
/**
 * Owned request and decoding context; contains credentials.
 */
class ComplianceRequest internal constructor (
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
            lib.fatoora_ComplianceRequest_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ComplianceRequestLib> = ComplianceRequestLib::class.java
        internal val lib: ComplianceRequestLib = Native.load("fatoora_ffi", libClass)
    }

    fun method(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ComplianceRequest_method(handle, write);
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

    fun url(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ComplianceRequest_url(handle, write);
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

    fun headersJson(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_ComplianceRequest_headers_json(handle, write);
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

    fun body(): Result<Bytes> {

        val returnVal = lib.fatoora_ComplianceRequest_body(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = Bytes(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun decode(status: UShort, body: UByteArray): Result<ComplianceCsid> {
        val bodySliceMemory = PrimitiveArrayTools.borrow(body)

        val returnVal = lib.fatoora_ComplianceRequest_decode(handle, FFIUint16(status), bodySliceMemory.slice);
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
            bodySliceMemory.close()
        }
    }

}
