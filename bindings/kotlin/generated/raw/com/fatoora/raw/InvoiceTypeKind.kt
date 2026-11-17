package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceTypeKindLib: Library {
}
/**
 * Kind of invoice document (UNCL1001 type code).
 *
 * Discriminants match the `u8` codes used before 0.6.
*/
enum class InvoiceTypeKind {
    Tax,
    Prepayment,
    CreditNote,
    DebitNote;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<InvoiceTypeKindLib> = InvoiceTypeKindLib::class.java
        internal val lib: InvoiceTypeKindLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): InvoiceTypeKind {
            return InvoiceTypeKind.entries[native]
        }

        fun default(): InvoiceTypeKind {
            return Tax
        }
    }
}
