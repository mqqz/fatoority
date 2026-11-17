package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface VatCategoryLib: Library {
}
/**
 * VAT category of a line, breakdown, or document-level adjustment.
 *
 * Discriminants match the `u8` codes used before 0.6.
*/
enum class VatCategory {
    Exempt,
    Standard,
    Zero,
    OutOfScope;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<VatCategoryLib> = VatCategoryLib::class.java
        internal val lib: VatCategoryLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): VatCategory {
            return VatCategory.entries[native]
        }

        fun default(): VatCategory {
            return Exempt
        }
    }
}
