package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ValidationLayerLib: Library {
}
/**
 * The validation layer that produced a finding.
 *
 * `Unknown` marks a layer this binding version does not recognize.
*/
enum class ValidationLayer {
    Unknown,
    FieldChecks,
    Xsd,
    BusinessRules,
    Signature,
    Qr,
    PreviousInvoiceHash;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ValidationLayerLib> = ValidationLayerLib::class.java
        internal val lib: ValidationLayerLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): ValidationLayer {
            return ValidationLayer.entries[native]
        }

        fun default(): ValidationLayer {
            return Unknown
        }
    }
}
