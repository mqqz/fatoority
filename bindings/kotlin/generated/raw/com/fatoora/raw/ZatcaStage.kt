package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZatcaStageLib: Library {
}
/**
 * A stage of the local ZATCA validation profile.
 *
 * `Unknown` marks a stage this binding version does not recognize.
*/
enum class ZatcaStage {
    Unknown,
    Xsd,
    Cen,
    Ksa,
    Signature,
    Qr,
    PreviousInvoiceHash;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ZatcaStageLib> = ZatcaStageLib::class.java
        internal val lib: ZatcaStageLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ZatcaStage {
            return ZatcaStage.entries[native]
        }

        fun default(): ZatcaStage {
            return Unknown
        }
    }
}
