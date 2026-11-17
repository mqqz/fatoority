package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceSubTypeLib: Library {
}
/**
 * Standard (B2B, cleared) or simplified (B2C, reported) invoice.
 *
 * Discriminants match the `u8` codes used before 0.6.
*/
enum class InvoiceSubType {
    Standard,
    Simplified;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<InvoiceSubTypeLib> = InvoiceSubTypeLib::class.java
        internal val lib: InvoiceSubTypeLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): InvoiceSubType {
            return InvoiceSubType.entries[native]
        }

        fun default(): InvoiceSubType {
            return Standard
        }
    }
}
