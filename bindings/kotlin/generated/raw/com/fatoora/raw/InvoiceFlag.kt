package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceFlagLib: Library {
    fun fatoora_InvoiceFlag_bit(inner: Int): FFIUint8
}
/**
 * Named bits of the `u8` invoice flag mask accepted by
 * `InvoiceBuilder::flags` and returned by `InvoiceData::flags_raw`.
 *
 * Each discriminant is the flag's bit value; `bit()` returns it as a `u8`
 * for languages whose enums do not expose their discriminant.
*/
enum class InvoiceFlag(val inner: Int) {
    ThirdParty(1),
    Nominal(2),
    Export(4),
    Summary(8),
    SelfBilled(16);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<InvoiceFlagLib> = InvoiceFlagLib::class.java
        internal val lib: InvoiceFlagLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): InvoiceFlag {
            return when (native) {
                1 -> ThirdParty
                2 -> Nominal
                4 -> Export
                8 -> Summary
                16 -> SelfBilled
                else -> throw RuntimeException("Failed to find variant ${native} of type InvoiceFlag")
            }
        }

        fun default(): InvoiceFlag {
            return ThirdParty
        }
    }

    /**
     * The flag's bit within the invoice flag mask.
     */
    fun bit(): UByte {

        val returnVal = lib.fatoora_InvoiceFlag_bit(this.toNative());
        return (returnVal.toUByte())
    }
}
