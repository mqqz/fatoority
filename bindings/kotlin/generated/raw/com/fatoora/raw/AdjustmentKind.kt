package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface AdjustmentKindLib: Library {
}
/**
 * Whether a document-level adjustment is an allowance (discount) or a charge.
*/
enum class AdjustmentKind {
    Allowance,
    Charge;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<AdjustmentKindLib> = AdjustmentKindLib::class.java
        internal val lib: AdjustmentKindLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): AdjustmentKind {
            return AdjustmentKind.entries[native]
        }

        fun default(): AdjustmentKind {
            return Allowance
        }
    }
}
