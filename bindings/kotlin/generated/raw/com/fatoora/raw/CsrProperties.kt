package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CsrPropertiesLib: Library {
    fun fatoora_CsrProperties_destroy(handle: Pointer)
    fun fatoora_CsrProperties_new(commonName: Slice, serialNumber: Slice, organizationIdentifier: Slice, organizationUnitName: Slice, organizationName: Slice, countryName: Slice, invoiceType: Slice, locationAddress: Slice, industryBusinessCategory: Slice): ResultPointerPointer
    fun fatoora_CsrProperties_from_properties_str(properties: Slice): ResultPointerPointer
    fun fatoora_CsrProperties_parse_csr_config_file(path: Slice): ResultPointerPointer
    fun fatoora_CsrProperties_build(handle: Pointer, key: Pointer, env: Int): ResultPointerPointer
}

class CsrProperties internal constructor (
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
            lib.fatoora_CsrProperties_destroy(handle)
    }

    companion object {
        internal val libClass: Class<CsrPropertiesLib> = CsrPropertiesLib::class.java
        internal val lib: CsrPropertiesLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(commonName: String, serialNumber: String, organizationIdentifier: String, organizationUnitName: String, organizationName: String, countryName: String, invoiceType: String, locationAddress: String, industryBusinessCategory: String): Result<CsrProperties> {
            val commonNameSliceMemory = PrimitiveArrayTools.borrowUtf8(commonName)
            val serialNumberSliceMemory = PrimitiveArrayTools.borrowUtf8(serialNumber)
            val organizationIdentifierSliceMemory = PrimitiveArrayTools.borrowUtf8(organizationIdentifier)
            val organizationUnitNameSliceMemory = PrimitiveArrayTools.borrowUtf8(organizationUnitName)
            val organizationNameSliceMemory = PrimitiveArrayTools.borrowUtf8(organizationName)
            val countryNameSliceMemory = PrimitiveArrayTools.borrowUtf8(countryName)
            val invoiceTypeSliceMemory = PrimitiveArrayTools.borrowUtf8(invoiceType)
            val locationAddressSliceMemory = PrimitiveArrayTools.borrowUtf8(locationAddress)
            val industryBusinessCategorySliceMemory = PrimitiveArrayTools.borrowUtf8(industryBusinessCategory)

            val returnVal = lib.fatoora_CsrProperties_new(commonNameSliceMemory.slice, serialNumberSliceMemory.slice, organizationIdentifierSliceMemory.slice, organizationUnitNameSliceMemory.slice, organizationNameSliceMemory.slice, countryNameSliceMemory.slice, invoiceTypeSliceMemory.slice, locationAddressSliceMemory.slice, industryBusinessCategorySliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = CsrProperties(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                commonNameSliceMemory.close()
                serialNumberSliceMemory.close()
                organizationIdentifierSliceMemory.close()
                organizationUnitNameSliceMemory.close()
                organizationNameSliceMemory.close()
                countryNameSliceMemory.close()
                invoiceTypeSliceMemory.close()
                locationAddressSliceMemory.close()
                industryBusinessCategorySliceMemory.close()
            }
        }
        @JvmStatic

        fun fromPropertiesStr(properties: String): Result<CsrProperties> {
            val propertiesSliceMemory = PrimitiveArrayTools.borrowUtf8(properties)

            val returnVal = lib.fatoora_CsrProperties_from_properties_str(propertiesSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = CsrProperties(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                propertiesSliceMemory.close()
            }
        }
        @JvmStatic

        fun parseCsrConfigFile(path: String): Result<CsrProperties> {
            val pathSliceMemory = PrimitiveArrayTools.borrowUtf8(path)

            val returnVal = lib.fatoora_CsrProperties_parse_csr_config_file(pathSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = CsrProperties(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                pathSliceMemory.close()
            }
        }
    }

    fun build(key: SigningKey, env: Environment): Result<Csr> {

        val returnVal = lib.fatoora_CsrProperties_build(handle, key.handle, env.toNative());
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = Csr(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
