package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface AddressLib: Library {
    fun fatoora_Address_destroy(handle: Pointer)
    fun fatoora_Address_with_province(handle: Pointer, value: Slice): ResultPointerPointer
    fun fatoora_Address_province(handle: Pointer): ResultPointerPointer
    fun fatoora_Address_new(countryCode: Slice, city: Slice, street: Slice, buildingNumber: Slice, postalCode: Slice, additionalStreet: OptionSlice, additionalNumber: OptionSlice, district: OptionSlice): ResultPointerPointer
    fun fatoora_Address_city(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Address_street(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Address_building_number(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Address_postal_code(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Address_country_code(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Address_additional_street(handle: Pointer): ResultPointerPointer
    fun fatoora_Address_additional_number(handle: Pointer): ResultPointerPointer
    fun fatoora_Address_district(handle: Pointer): ResultPointerPointer
}

class Address internal constructor (
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
            lib.fatoora_Address_destroy(handle)
    }

    companion object {
        internal val libClass: Class<AddressLib> = AddressLib::class.java
        internal val lib: AddressLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(countryCode: String, city: String, street: String, buildingNumber: String, postalCode: String, additionalStreet: String?, additionalNumber: String?, district: String?): Result<Address> {
            val countryCodeSliceMemory = PrimitiveArrayTools.borrowUtf8(countryCode)
            val citySliceMemory = PrimitiveArrayTools.borrowUtf8(city)
            val streetSliceMemory = PrimitiveArrayTools.borrowUtf8(street)
            val buildingNumberSliceMemory = PrimitiveArrayTools.borrowUtf8(buildingNumber)
            val postalCodeSliceMemory = PrimitiveArrayTools.borrowUtf8(postalCode)
            val additionalStreetSliceMemory = additionalStreet?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val additionalNumberSliceMemory = additionalNumber?.let { PrimitiveArrayTools.borrowUtf8(it) }
            val districtSliceMemory = district?.let { PrimitiveArrayTools.borrowUtf8(it) }

            val returnVal = lib.fatoora_Address_new(countryCodeSliceMemory.slice, citySliceMemory.slice, streetSliceMemory.slice, buildingNumberSliceMemory.slice, postalCodeSliceMemory.slice, additionalStreetSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), additionalNumberSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none(), districtSliceMemory?.let { OptionSlice.some(it.slice) } ?: OptionSlice.none());
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = Address(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                countryCodeSliceMemory.close()
                citySliceMemory.close()
                streetSliceMemory.close()
                buildingNumberSliceMemory.close()
                postalCodeSliceMemory.close()
                additionalStreetSliceMemory?.close()
                additionalNumberSliceMemory?.close()
                districtSliceMemory?.close()
            }
        }
    }

    fun withProvince(value: String): Result<Address> {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)

        val returnVal = lib.fatoora_Address_with_province(handle, valueSliceMemory.slice);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = Address(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        } finally {
            valueSliceMemory.close()
        }
    }

    fun province(): Result<Text?> {

        val returnVal = lib.fatoora_Address_province(handle);
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

    fun city(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Address_city(handle, write);
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

    fun street(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Address_street(handle, write);
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

    fun buildingNumber(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Address_building_number(handle, write);
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

    fun postalCode(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Address_postal_code(handle, write);
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

    fun countryCode(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Address_country_code(handle, write);
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

    fun additionalStreet(): Result<Text?> {

        val returnVal = lib.fatoora_Address_additional_street(handle);
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

    fun additionalNumber(): Result<Text?> {

        val returnVal = lib.fatoora_Address_additional_number(handle);
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

    fun district(): Result<Text?> {

        val returnVal = lib.fatoora_Address_district(handle);
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

}
