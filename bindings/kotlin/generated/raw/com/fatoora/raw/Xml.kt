package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface XmlLib: Library {
    fun fatoora_Xml_destroy(handle: Pointer)
    fun fatoora_Xml_validate_zatca(xml: Slice, optionsJson: OptionSlice): ResultPointerPointer
    fun fatoora_Xml_validate(xml: Slice): ResultBytePointer
    fun fatoora_Xml_hash(xml: Slice, write: Pointer): ResultUnitPointer
}

class Xml internal constructor (
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
            lib.fatoora_Xml_destroy(handle)
    }

    companion object {
        internal val libClass: Class<XmlLib> = XmlLib::class.java
        internal val lib: XmlLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        /**
         * Run the local ZATCA validation profile and return its owned report.
         *
         * Inspect `is_valid` before acceptance. Options default when absent;
         * `options_json` is limited to 4 KiB. Validation needs no configuration:
         * it mirrors the stateless core `Validator::validate_zatca`.
         */
        fun validateZatca(xml: String, optionsJson: String?): Result<ZatcaValidationReport> {
            val xmlSliceMemory = PrimitiveArrayTools.borrowUtf8(xml)
            val optionsJsonSliceMemory = optionsJson?.let { PrimitiveArrayTools.borrowUtf8(it) }

            val returnVal = lib.fatoora_Xml_validate_zatca(xmlSliceMemory.slice, optionsJsonSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = ZatcaValidationReport(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                xmlSliceMemory.close()
                optionsJsonSliceMemory?.close()
            }
        }
        @JvmStatic

        /**
         * Check the XML against the UBL schema and business rules, mirroring core
         * `Validator::validate_xml`. Returns true or a validation error.
         */
        fun validate(xml: String): Result<Boolean> {
            val xmlSliceMemory = PrimitiveArrayTools.borrowUtf8(xml)

            val returnVal = lib.fatoora_Xml_validate(xmlSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    return (nativeOkVal > 0).ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                xmlSliceMemory.close()
            }
        }
        @JvmStatic

        fun hash(xml: String): Result<String> {
            val xmlSliceMemory = PrimitiveArrayTools.borrowUtf8(xml)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.fatoora_Xml_hash(xmlSliceMemory.slice, write);
            try {
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
            } finally {
                xmlSliceMemory.close()
            }
        }
    }

}
